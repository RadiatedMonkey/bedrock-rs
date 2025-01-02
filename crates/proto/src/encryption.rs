
use base64::{engine::general_purpose::STANDARD, Engine};
use p384::{self, ecdh::diffie_hellman, pkcs8::DecodePublicKey, PublicKey, SecretKey};

use sha2::{Sha256, Digest};
use rand::Rng;

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};

use bedrockrs_proto_core::error::EncryptionError;
use crate::v729::packets::login::LoginPacket;

#[derive(Clone)]
pub struct Encryption {
    recv_counter: u64,
    send_counter: u64,
    buf: [u8; 8],
    key: Vec<u8>,
    iv: Vec<u8>,
    cipher: Aes256Gcm,
}

impl std::fmt::Debug for Encryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Encryption")
            .field("send_counter", &self.send_counter)
            .field("buf", &self.buf)
            .field("key", &self.key)
            .field("iv", &self.iv)
            .finish()
    }
}

//Reversed from bedrock dedicated server v 1.21.51.02 
impl Encryption {
    pub fn new() -> Self {
        let key = vec![0u8; 32]; // Initialize with 32 zero bytes (can be set to your desired key)

        let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&key));

        Encryption{
            send_counter: 0,
            buf: [0; 8],
            key: Vec::new(),
            iv: Vec::new(),
            cipher,
        }
    }

    pub fn decrypt(&mut self, ciphertext: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        // In bedrock dedicated server, there are serveral encryption method, but it turned out they are likely to always use
        // AES-256-GCM
        let nonce = Nonce::from_slice(&self.iv);

        self.cipher.decrypt(nonce, ciphertext.as_slice())
            .map_err(|_| EncryptionError::DecryptionFailed())
    }

    pub fn encrypt(&mut self, plaintext: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        let nonce = Nonce::from_slice(&self.iv);

        self.cipher.encrypt(nonce, plaintext.as_slice())
            .map_err(|_| EncryptionError::EncryptionFailed())
    }

    pub fn verify(&mut self, _src: &[u8]) -> Result<(), EncryptionError> {
        unimplemented!()
    }

    pub fn get_ident_key(&mut self, login_packet: &LoginPacket) -> Result<String, EncryptionError> {
        let cert_chain = &login_packet.connection_request.certificate_chain;

        let last_chain = cert_chain.last().unwrap();

        let identity_public_key = last_chain.get("identityPublicKey").unwrap();

        return Ok(identity_public_key.clone().to_string()); 
    }

    pub fn compute_shared_secret_ecc(
        &mut self, 
        server_private_key: &[u8],
        in_public_key: &[u8]
    )->Result<Vec<u8>, EncryptionError>{
        let server_private = SecretKey::from_sec1_der(server_private_key)
            .map_err(|_| EncryptionError::StartupFailed())?;
        
        let peer_public = PublicKey::from_public_key_der(in_public_key).unwrap();

        let secret = diffie_hellman(server_private.to_nonzero_scalar(), peer_public.as_affine());

        Ok(secret.raw_secret_bytes().to_vec())
    }

    pub fn init_encryption(&mut self, server_private_key: Vec<u8>, login_packet: LoginPacket) -> Result<Vec<u8>, EncryptionError> {

        //The "identityPublicKey from the last part of the chain will be used as encryption seed
        let identity_publickey = self.get_ident_key(&login_packet)
            .map_err(|_| EncryptionError::MissingKey)?;

        //Decode the peer public key using base64
        let peer_pub_key_der = STANDARD.decode(identity_publickey).unwrap();
        let shared_secret = self.compute_shared_secret_ecc(server_private_key.as_slice(), &peer_pub_key_der)
            .map_err(|_| EncryptionError::StartupFailed())?;

        //Generate 16-byte random slice for the first 8 byte
        let mut rng = rand::thread_rng();
        let mut final_key_seed : Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        final_key_seed.extend_from_slice(&shared_secret);


        //Reversed from bds, uses sha-256 for applying hash
        let mut hasher = Sha256::new();
        hasher.update(final_key_seed);

        let encryption_symmetric_key = hasher.finalize().to_vec();

        //Note that after some reversing, I notice that the first 16 byte of the key will also be used as IV
        self.key = encryption_symmetric_key.clone();
        self.iv = encryption_symmetric_key[0..16].to_vec();

        self.cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&self.key.as_slice()));

        Ok(encryption_symmetric_key)
    }
}
