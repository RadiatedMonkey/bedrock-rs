
use base64::engine::general_purpose;
use base64::Engine;
use openssl::bn::BigNumContext;
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::ec::{EcGroup, EcKey, EcPoint};
use openssl::pkey::PKey;
use openssl::derive::Deriver;
use openssl::nid::Nid;

use sha2::{Sha256, Digest};
use rand::Rng;

use bedrockrs_proto_core::error::EncryptionError;
use crate::v729::packets::login::LoginPacket;

#[derive(Debug, Clone)]
pub struct Encryption {
    send_counter: u64,
    buf: [u8; 8],
    key: Vec<u8>,
    iv: Vec<u8>,
}

//Reversed from bedrock dedicated server v 1.21.51.02 
impl Encryption {
    pub fn new() -> Self {
        Encryption{
            send_counter: 0,
            buf: [0; 8],
            key: Vec::new(),
            iv: Vec::new()
        }
    }

    pub fn decrypt(&mut self, _src: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {

        // In bedrock dedicated server, there are serveral encryption method, but it turned out they are likely to always use
        // AES-256-GCM

        let cipher = Cipher::aes_256_gcm();

        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &self.key, Some(&self.iv))
            .map_err(|_| EncryptionError::DecryptionFailed())?;

        let mut decrypted = vec![0u8; _src.len()];
        let len = crypter.update(&_src, &mut decrypted).map_err(|_| EncryptionError::DecryptionFailed())?;

        let final_len = crypter.finalize(&mut decrypted[len..]).map_err(|_| EncryptionError::DecryptionFailed())?;
        decrypted.truncate(len + final_len);
        Ok(decrypted)
    }

    pub fn encrypt(&mut self, _src: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        let cipher = Cipher::aes_256_gcm();

        let mut crypter = Crypter::new(cipher, Mode::Encrypt, &self.key, Some(&self.iv))
        .map_err(|_| EncryptionError::EncryptionFailed())?;

        let mut ciphertext = vec![0u8; _src.len() + cipher.block_size()];

        let len = crypter.update(&_src, &mut ciphertext).map_err(|_| EncryptionError::EncryptionFailed())?;
        let final_len = crypter.finalize(&mut ciphertext[len..]).map_err(|_| EncryptionError::EncryptionFailed())?;

        ciphertext.truncate(len + final_len);
        Ok(ciphertext)
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

        let private_key = EcKey::private_key_from_der(server_private_key)
            .map_err(|_| EncryptionError::StartupFailed())?;

        let private_key : PKey<_>  = private_key.try_into().unwrap();

        let group = EcGroup::from_curve_name(Nid::SECP384R1)
            .map_err(|_| EncryptionError::StartupFailed())?;
        let mut ctx = BigNumContext::new()
            .map_err(|_| EncryptionError::StartupFailed())?;

        let point = EcPoint::from_bytes(&group, &in_public_key, &mut ctx).unwrap();
        
        let recipient_key : PKey<_> = EcKey::from_public_key(&group, &point).unwrap().try_into()
            .map_err(|_| EncryptionError::StartupFailed())?;

        let mut deriver = Deriver::new(&private_key)
            .map_err(|_| EncryptionError::StartupFailed())?;
        
        deriver.set_peer(&recipient_key).unwrap();
        let secret = deriver.derive_to_vec().unwrap();

        Ok(secret)
    }

    pub fn init_encryption(&mut self, server_private_key: Vec<u8>, login_packet: LoginPacket) -> Result<Vec<u8>, EncryptionError> {

        //The "identityPublicKey from the last part of the chain will be used as encryption seed
        let identity_publickey = self.get_ident_key(&login_packet)
            .map_err(|_| EncryptionError::MissingKey)?;

        //Decode the peer public key using base64
        let peer_pub_key_der = general_purpose::STANDARD.decode(identity_publickey).unwrap();
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

        Ok(encryption_symmetric_key)
    }
}
