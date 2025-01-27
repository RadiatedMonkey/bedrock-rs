use bedrockrs_proto::error::ConnectionError;
use thiserror::Error;

pub enum StartError {}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Connection Error: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Login aborted, reason: {reason}")]
    Abort { reason: String },
    #[error("Wrong protocol version (client: {client}, server: {server:?})")]
    PVNMismatch { client: i32, server: &'static [i32] },
    #[error("Format Error: {0}")]
    FormatError(&'static str),
}

pub enum ServerError {}
