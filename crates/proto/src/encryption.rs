use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bedrockrs_proto_core::error::{EncryptionError, ProtoCodecError};
use ctr::cipher::{KeyIvInit, StreamCipher};
use jsonwebtoken::Algorithm;
use p384::ecdh::diffie_hellman;
use p384::ecdsa::SigningKey;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use p384::PublicKey;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::{self, Debug};
use std::io::Write;

type Aes256CtrBE = ctr::Ctr64BE<aes::Aes256>;

#[derive(Serialize)]
struct EncryptionClaims {
    salt: String,
}

#[derive(Clone)]
pub struct Encryption {
    recv_counter: u64,
    send_counter: u64,
    jwt: String,

    decrypt: Aes256CtrBE,
    encrypt: Aes256CtrBE,
    secret: [u8; 32],
}

impl Debug for Encryption {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Encryption")
            .field("jwt", &self.jwt)
            .finish_non_exhaustive()
    }
}

impl Encryption {
    pub fn new(client_public_key_der: &str) -> Result<Self, ProtoCodecError> {
        let salt = (0..16)
            .map(|_| OsRng.sample(Alphanumeric) as char)
            .collect::<String>();
        let private_key: SigningKey = SigningKey::random(&mut OsRng);

        let Ok(private_key_der) = private_key.to_pkcs8_der() else {
            todo!()
        };

        let public_key = private_key.verifying_key();
        let public_key_der = if let Ok(key) = public_key.to_public_key_der() {
            BASE64_STANDARD.encode(key)
        } else {
            todo!()
        };

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.typ = None;
        header.x5u = Some(public_key_der);

        let signing_key = jsonwebtoken::EncodingKey::from_ec_der(&private_key_der.to_bytes());
        let claims = EncryptionClaims {
            salt: BASE64_STANDARD.encode(&salt),
        };

        let jwt = jsonwebtoken::encode(&header, &claims, &signing_key)?;

        let bytes = BASE64_STANDARD.decode(client_public_key_der)?;
        let Ok(client_public_key) = PublicKey::from_public_key_der(&bytes) else {
            todo!()
        };

        let shared_secret = diffie_hellman(
            private_key.as_nonzero_scalar(),
            client_public_key.as_affine(),
        );

        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(shared_secret.raw_secret_bytes().as_slice());

        let secret = hasher.finalize();

        let mut iv = [0; 16];
        iv[..12].copy_from_slice(&secret[..12]);
        iv[12..].copy_from_slice(&[0x00, 0x00, 0x00, 0x02]);

        let cipher = Aes256CtrBE::new((&secret).into(), (&iv).into());

        Ok(Self {
            send_counter: 0,
            recv_counter: 0,

            decrypt: cipher.clone(),
            encrypt: cipher,
            secret: secret.into(),

            jwt,
        })
    }

    #[inline]
    pub fn salt_jwt(&self) -> &str {
        &self.jwt
    }

    pub fn decrypt(&mut self, data: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        dbg!("Decrypting");
        println!("data: {:?}", &data[..10]);

        if data.len() < 9 {
            // This data cannot possibly be valid. Checksum is already 8 bytes.
            todo!()
        }

        self.decrypt.apply_keystream(data);
        println!("decrypt: {data:?}");

        let counter = self.recv_counter;
        self.recv_counter += 1;

        let checksum = &data[data.len() - 8..];
        let computed = self.checksum(&data[..data.len() - 8], counter);
        if !checksum.eq(&computed) {
            panic!("Checksum does not match")
        }

        data.truncate(data.len() - 8);

        Ok(())
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let counter = self.send_counter;
        self.send_counter += 1;

        let checksum = self.checksum(data, counter);
        data.write_all(&checksum)?;

        self.encrypt.apply_keystream(data);

        Ok(())
    }

    pub fn checksum(&self, data: &[u8], counter: u64) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(counter.to_le_bytes());
        hasher.update(data);
        hasher.update(&self.secret);

        let mut checksum = [0; 8];
        checksum.copy_from_slice(&hasher.finalize()[..8]);

        checksum
    }
}
