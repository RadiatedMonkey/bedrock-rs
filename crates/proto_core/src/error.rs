use std::convert::Infallible;
use std::error::Error;
use std::io::Error as IOError;
use std::num::{ParseIntError, TryFromIntError};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use base64::DecodeError as Base64DecodeError;
use jsonwebtoken::errors::Error as JwtError;
use nbtx::NbtError;
use p384::pkcs8::spki;
use serde_json::error::Error as JsonError;
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Error, Debug)]
pub enum ProtoCodecError {
    #[error("IOError occurred: {0}")]
    IOError(#[from] IOError),
    #[error("Unread bytes remaining: {0} bytes left")]
    LeftOvers(usize),
    #[error("NbtError: {0}")]
    NbtError(#[from] NbtError),
    #[error("Error while reading UTF-8 encoded String: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Error while converting integers: {0}")]
    FromIntError(#[from] TryFromIntError),
    #[error("Json Error: {0}")]
    JsonError(#[from] JsonError),
    #[error("Jwt Error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Uuid Error: {0}")]
    UuidError(#[from] UuidError),
    #[error("Base64 decoding Error: {0}")]
    Base64DecodeError(#[from] Base64DecodeError),
    #[error("XUID could not be parsed : {0}")]
    XuidParseError(#[from] ParseIntError),
    /// TODO: This likely hurts performance, but it is *kinda* good for debugging
    #[error("parse value `{0}` to enum variant for {1} enum")]
    InvalidEnumID(String, &'static str),
    #[error("Got an unknown/invalid game packet id: {0}")]
    InvalidGamePacketID(u16),
    #[error("Expected format got mismatched: {0}")]
    FormatMismatch(&'static str),
    #[error("Compression Error: {0}")]
    CompressError(#[from] CompressionError),
    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] EncryptionError),
    #[error("Login error: {0}")]
    LoginError(#[from] LoginError),
}

impl From<Infallible> for ProtoCodecError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Zlib Error: {0}")]
    ZlibError(#[from] Box<dyn Error + Send + Sync>),
    #[error("Snappy Error: {0}")]
    SnappyError(#[from] IOError),
    #[error("Unknown Compression Method: {0}")]
    UnknownCompressionMethod(u8),
    #[error("IO Error: {0}")]
    IOError(IOError),
}

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("IO Error: {0}")]
    IOError(IOError),

    #[error("Encryption Error")]
    EncryptionFailed(),

    #[error("Decryption Error")]
    DecryptionFailed(),

    #[error("Startup Error")]
    StartupFailed(),

    #[error("Missing Key")]
    MissingKey,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Missing X5U header in JWT")]
    MissingX5U,
    #[error("Invalid chain length: {0}")]
    InvalidChainLength(usize),
    #[error("Authentication token not signed by Mojang")]
    NotSignedByMojang,
    #[error("User is not authenticated with Xbox services")]
    UserOffline,
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(spki::Error),
}
