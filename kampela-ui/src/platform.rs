//! Platform definitions
//!
//! Simplified for Ethereum-only functionality with Result-based error handling.

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature="std")]
use std::{string::String, vec::Vec};

use rand::{CryptoRng, Rng};
use alloy_primitives::Address;

use crate::error::KampelaError;

pub type PinCode = [u8; 4];

/// Length of entropy (32 bytes = 256 bits for Ethereum private key)
pub const ENTROPY_LEN: usize = 32;

/// Platform abstraction trait for Kampela firmware.
///
/// Implement this on your platform (device or simulator) to provide
/// hardware access, cryptographic RNG, and persistent storage.
pub trait Platform {
    /// Hardware abstraction layer handle.
    ///
    /// Passed to operations requiring hardware access. Should be used within
    /// critical sections/mutex locks.
    type HAL;

    /// Cryptographically secure random number generator.
    type Rng<'a>: Rng + Sized + CryptoRng;

    /// Ethereum transaction type (platform-specific representation).
    type EthTransaction;

    /// Get a cryptographic RNG instance from the HAL.
    fn rng(h: &mut Self::HAL) -> Self::Rng<'_>;

    /// Get immutable reference to PIN code.
    fn pin(&self) -> &PinCode;

    /// Get mutable reference to PIN code.
    fn pin_mut(&mut self) -> &mut PinCode;

    /// Store entropy (32-byte private key) to persistent storage (flash).
    ///
    /// On device: encrypts with AES-GCM and writes to flash.
    /// On simulator: stores in memory.
    fn store_entropy(&mut self, e: &[u8]) -> Result<(), KampelaError>;

    /// Read entropy from persistent storage into memory.
    ///
    /// On device: reads from flash and decrypts.
    /// On simulator: reads from memory.
    fn read_entropy(&mut self) -> Result<(), KampelaError>;

    /// Get the raw 32-byte private key (entropy).
    fn entropy(&self) -> Result<Vec<u8>, KampelaError>;

    /// Get the Ethereum address derived from entropy.
    fn eth_address(&self) -> Result<Address, KampelaError>;

    /// Set the Ethereum address (caches derived address).
    fn eth_set_address(&mut self, addr: Address);

    /// Set the current Ethereum transaction to be signed.
    fn eth_set_transaction(&mut self, transaction: Self::EthTransaction);

    /// Get reference to the current Ethereum transaction.
    fn eth_transaction(&self) -> Result<&Self::EthTransaction, KampelaError>;

    /// Get formatted display string for the current transaction (clear-signing).
    fn eth_transaction_display(&self) -> Result<String, KampelaError>;

    /// Sign the current Ethereum transaction and return signed raw transaction bytes.
    fn eth_sign_transaction(&mut self) -> Result<Vec<u8>, KampelaError>;

    /// Generate a new random 32-byte entropy (private key).
    ///
    /// Uses the platform's cryptographic RNG.
    fn generate_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN] = [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }
}
