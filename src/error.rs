//! error.rs
//! Error type definitions
//! Defines error types based on the thiserror crate, providing detailed error information and context
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    // ---- From AuthError ----
    #[error("hex decode failed: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("invalid private key bytes")]
    InvalidPrivateKey,

    #[error("invalid public key bytes")]
    InvalidPublicKey,

    #[error("signature parse error")]
    SignatureParse,

    #[error("signature verify failed")]
    VerifyFailed,

    // ---- From AuthError ----
    #[error("command execute failed: {0}")]
    CmdFail(String),

    #[error("regex capture match not found")]
    RegexMatch,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Simplified Result type alias
pub type Result<T> = std::result::Result<T, AuthError>;