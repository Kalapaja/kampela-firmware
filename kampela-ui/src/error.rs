//! Error types for Kampela Ethereum-only firmware
//!
//! All operations return Result types to enable proper error display
//! on the device screen (no USB/USART for logs).

use core::fmt;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use std::string::{String, ToString};

/// Main error type for Kampela operations
#[derive(Debug, Clone)]
pub enum KampelaError {
    /// Flash read operation failed
    FlashRead,

    /// Flash write operation failed (includes verification failure)
    FlashWrite,

    /// Private key generation or loading failed
    KeyGeneration,

    /// Invalid Ethereum transaction data
    TransactionInvalid(String),

    /// Transaction signing operation failed
    SigningFailed,

    /// NFC communication or parsing error
    NfcError(String),

    /// PIN verification failed
    PinIncorrect,

    /// Encryption/decryption error
    CryptoError,

    /// Generic error with message
    Other(String),
}

impl fmt::Display for KampelaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KampelaError::FlashRead => write!(f, "Flash read failed"),
            KampelaError::FlashWrite => write!(f, "Flash write failed"),
            KampelaError::KeyGeneration => write!(f, "Key generation failed"),
            KampelaError::TransactionInvalid(msg) => write!(f, "Invalid TX: {}", msg),
            KampelaError::SigningFailed => write!(f, "Signing failed"),
            KampelaError::NfcError(msg) => write!(f, "NFC error: {}", msg),
            KampelaError::PinIncorrect => write!(f, "Incorrect PIN"),
            KampelaError::CryptoError => write!(f, "Crypto error"),
            KampelaError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<&str> for KampelaError {
    fn from(s: &str) -> Self {
        KampelaError::Other(s.to_string())
    }
}

/// Helper macro for creating errors with formatted messages
#[macro_export]
macro_rules! kampela_error {
    ($variant:ident, $($arg:tt)*) => {{
        let mut msg = String::new();
        use core::fmt::Write;
        let _ = write!(&mut msg, $($arg)*);
        KampelaError::$variant(msg)
    }};
}

pub use kampela_error;
