//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature="std")]
use std::{string::String, vec::Vec};

use rand::{CryptoRng, Rng};

use substrate_crypto_light::{common::cut_path, sr25519::{Pair, Public}};
use substrate_parser::{TransactionUnmarkedParsed, ShortSpecs};
use mnemonic_external::AsWordList;

pub type PinCode = [u8; 4];
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
    
    /// Getter for seed
    fn entropy(&self) -> Option<Vec<u8>>;

    fn set_derivation(&mut self, path: Vec<u8>);

    fn set_transaction(&mut self, transaction: Self::NfcTransaction);

    fn call(&mut self) -> Option<String>;

    fn extensions(&mut self) -> Option<String>;

    fn signature(&mut self) -> [u8; 130];

    fn derivation(&self) -> &str;

    fn read_derivation(&mut self);

    fn store_derivation(&mut self);

    //----derivatives----

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }

    fn pair(&self, h: &mut Self::HAL) -> Option<Pair> {
        let e = self.entropy()?;
        if e.is_empty() { None } else {
            let a = self.derivation();
            let full_derivation = cut_path(a).unwrap();
            Pair::from_entropy_and_full_derivation_external_rng(&e, full_derivation, &mut Self::rng(h)).ok()
        }
    }

    fn public(&self, h: &mut Self::HAL) -> Option<Public> {
        self.pair(h).map(|pair| pair.public())
    }

}

pub struct NfcTransaction {
    pub decoded_transaction: TransactionUnmarkedParsed,
    pub data_to_sign: Vec<u8>,
    pub specs: ShortSpecs,
    pub spec_name: String,
}
