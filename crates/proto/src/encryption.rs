use std::fmt::{self, Debug};

use aes::cipher::{KeyIvInit, StreamCipher};
use base64::{prelude::BASE64_STANDARD, Engine};
use bedrockrs_proto_core::error::{EncryptionError, ProtoCodecError};
use jsonwebtoken::Algorithm;
use p384::ecdh::diffie_hellman;
use p384::ecdsa::SigningKey;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use p384::PublicKey;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::Serialize;
use sha2::{Digest, Sha256};

type Aes256CtrBe = ctr::Ctr64BE<aes::Aes256>;

#[derive(Serialize)]
struct EncryptionClaims {
    salt: String
}

#[derive(Clone)]
pub struct Encryptor {
    decrypt: Aes256CtrBe,
    encrypt: Aes256CtrBe,
    send_counter: u64,
    recv_counter: u64,
    secret: [u8; 32],
    pub jwt: String
}

impl Debug for Encryptor {
    // Don't reveal secrets
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt
            .debug_struct("Encryptor")
            .field("jwt", &self.jwt)
            .finish_non_exhaustive()
    }
}

impl Encryptor {
    /// The only argument to this function is the client's DER-encoded public key
    /// which is obtained in the third JWT in the login chain.
    pub fn new(client_public_key_der: &str) -> Result<Self, ProtoCodecError> {
        let salt = (0..16).map(|_| OsRng.sample(Alphanumeric) as char).collect::<String>();
        let private_key: SigningKey = SigningKey::random(&mut OsRng);
        let Ok(private_key_der) = private_key.to_pkcs8_der() else {
            unimplemented!();
        };

        let public_key = private_key.verifying_key();
        let public_key_der = if let Ok(k) = public_key.to_public_key_der() {
            BASE64_STANDARD.encode(k)
        } else {
            unimplemented!();
        };

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.typ = None;
        header.x5u = Some(public_key_der);

        let signing_key = jsonwebtoken::EncodingKey::from_ec_der(&private_key_der.to_bytes());
        let claims = EncryptionClaims { salt: BASE64_STANDARD.encode(&salt) };
        
        let jwt = jsonwebtoken::encode(&header, &claims, &signing_key)?;
        let bytes = BASE64_STANDARD.decode(client_public_key_der)?;
        let Ok(client_public_key) = PublicKey::from_public_key_der(&bytes) else {
            unimplemented!();
        };

        let shared_secret = diffie_hellman(
            private_key.as_nonzero_scalar(), 
            client_public_key.as_affine()  
        );

        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(shared_secret.raw_secret_bytes().as_slice());

        let mut secret = [0u8; 32];
        secret.copy_from_slice(&hasher.finalize()[0..32]);

        let mut iv = [0u8; 16];
        iv[..12].copy_from_slice(&secret[..12]);
        iv[12..].copy_from_slice(&[0x00, 0x00, 0x00, 0x02]);

        let cipher = Aes256CtrBe::new((&secret).into(), (&iv).into());
        
        Ok(Self {
            send_counter: 0,
            recv_counter: 0,
            decrypt: cipher.clone(),
            encrypt: cipher,
            secret,
            jwt
        })
    }

    pub fn decrypt(&mut self, src: &mut Vec<u8>) -> Result<(), EncryptionError> {
        self.decrypt.apply_keystream(src);
        
        let count = self.recv_counter;
        self.recv_counter += 1;

        let checksum = &src[src.len() - 8..];
        let computed_checksum = self.checksum(&src[..src.len() - 8], count);
        if !checksum.eq(&computed_checksum) {
            todo!();
        }

        src.truncate(src.len() - 8);
        Ok(())
    }

    pub fn encrypt(&mut self, src: &mut Vec<u8>) -> Result<(), EncryptionError> {
        let count = self.send_counter;
        self.send_counter += 1;

        let checksum = self.checksum(&src, count);
        src.extend(checksum);

        self.encrypt.apply_keystream(src);

        Ok(())
    }

    pub fn checksum(&mut self, src: &[u8], count: u64) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(count.to_le_bytes());
        hasher.update(src);
        hasher.update(&self.secret);

        let mut checksum = [0u8; 8];
        checksum.copy_from_slice(&hasher.finalize()[..8]);

        checksum
    }
}
