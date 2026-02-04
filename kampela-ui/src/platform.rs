//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature="std")]
use std::{string::String, vec::Vec};

use rand::{CryptoRng, Rng};

use substrate_crypto_light::sr25519::{Pair, Public};
use substrate_parser::{ShortSpecs, TransactionUnmarkedParsed};

use mnemonic_external::AsWordList;

pub type PinCode = [u8; 4];
pub type EthAddress = [u8; 20];
const ENTROPY_LEN: usize = 32; //TODO: move to appropriate place

/// Implement this on platform to make crate work
pub trait Platform {
    /// Peripherals access should be external to this type since it is used elsewhere in general;
    /// Thus an external object HAL would be passed to all operations. Generally it should happen
    /// within mutex lock, so make sure to set up some kind of critical section aroung this object.
    type HAL;

    /// Sufficiently good random source used everywhere
    type Rng<'a>: Rng + Sized + CryptoRng;

    /// Transaction data or addresses for transaction data in psram
    type NfcTransaction;

    type EthTransaction;

    /// List-set of mnemonic words 
    type AsWordList: AsWordList;
    // Device-specific wordlist implementation
    fn get_wordlist() -> Self::AsWordList;

    /// RNG getter
    fn rng(h: &mut Self::HAL) -> Self::Rng<'_>;

    /// Device-specific "global" storage and management of pincode state RO
    fn pin(&self) -> &PinCode;

    /// Device-specific "global" storage and management of pincode state RW
    fn pin_mut(&mut self) -> &mut PinCode;

    /// Put entropy in flash
    fn store_entropy(&mut self, e: &[u8]);

    /// Read entropy from flash
    fn read_entropy(&mut self);

    /// Getter for public address
    fn sub_public(&self) -> Option<Public>;

    /// Getter for seed
    fn entropy(&self) -> Option<Vec<u8>>;

    fn sub_set_address(&mut self, addr: [u8; 76]);

    fn sub_set_transaction(&mut self, transaction: Self::NfcTransaction);

    fn sub_call(&mut self) -> Option<String>;

    fn sub_extensions(&mut self) -> Option<String>;

    fn sub_signature(&mut self) -> [u8; 130];

    fn sub_address(&self) -> Option<&[u8; 76]>;

    fn eth_address(&self) -> Option<EthAddress>;

    fn eth_set_address(&mut self, addr: EthAddress);

    fn eth_set_transaction(&mut self, transaction: Self::EthTransaction);

    fn eth_transaction(&self) -> Option<&Self::EthTransaction>;

    fn eth_transaction_display(&self) -> Option<String>;

    fn eth_sign_transaction(&mut self) -> Option<Vec<u8>>;

    //----derivatives----

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }

    fn sub_pair(&self) -> Option<Pair> {
        let e = self.entropy()?;
        if e.is_empty() { None } else {
            Pair::from_entropy_and_pwd(&e, "").ok()
        }
    }
}

pub struct NfcTransaction {
    pub decoded_transaction: TransactionUnmarkedParsed,
    pub data_to_sign: Vec<u8>,
    pub specs: ShortSpecs,
    pub spec_name: String,
}
