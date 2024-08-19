//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature="std")]
use std::{string::String, vec::Vec};

use rand::{CryptoRng, Rng};

use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use schnorrkel::{
    keys::Keypair,
    ExpansionMode,
    MiniSecretKey,
};

use mnemonic_external::{AsWordList, Bits11, ErrorWordList, WordSet, TOTAL_WORDS};

pub type PinCode = [u8; 4]; //TODO: consider if it's good password storing type
const ENTROPY_LEN: usize = 32; //TODO: move to appropriate place

/// Implement this on platform to make crate work
pub trait Platform {
    /// Peripherals access should be external to this type since it is used elsewhere in general;
    /// Thus an external object HAL would be passed to all operations. Generally it should happen
    /// within mutex lock, so make sure to set up some kind of critical section aroung this object.
    type HAL;

    /// Sufficiently good random source used everywhere
    type Rng: Rng + Sized + CryptoRng;


    /// Transaction data or addresses for transaction data in psram
    type NfcTransaction;

    /// List-set of mnemonic words 
    type AsWordList: AsWordList + ?Sized;

    /// RNG getter
    fn rng<'a>(h: &'a mut Self::HAL) -> &'a mut Self::Rng;

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

    fn public(&self) -> Option<[u8; 32]>;

    fn set_address(&mut self, addr: [u8; 76]);

    fn set_transaction(&mut self, transaction: Self::NfcTransaction);

    fn call(&mut self) -> Option<String>;

    fn extensions(&mut self) -> Option<String>;

    fn signature(&mut self, h: &mut Self::HAL) -> [u8; 130];

    fn address(&mut self) -> &[u8; 76];

    //----derivatives----

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }

    fn pair(&self) -> Option<Keypair> {
        match self.entropy() {
            None => None,
            Some(e) => pair_from_entropy(&e),
        }
    }
}

pub fn entropy_to_big_seed(entropy: &[u8]) -> [u8; 64] {
    //check_entropy_length(entropy)?;

    let salt = "mnemonic";

    let mut seed = [0u8; 64];

    pbkdf2::<Hmac<Sha512>>(entropy, salt.as_bytes(), 2048, &mut seed);

    seed
}


pub fn pair_from_entropy(e: &[u8]) -> Option<Keypair> {
    if e.is_empty() { None } else {
        let big_seed = entropy_to_big_seed(&e);

        let mini_secret_bytes = &big_seed[..32];

        Some(
            MiniSecretKey::from_bytes(mini_secret_bytes)
                .unwrap()
                .expand_to_keypair(ExpansionMode::Ed25519)
        )
    }
}

pub fn public_from_entropy(e: &[u8]) -> Option<[u8; 32]> {
    let pair = pair_from_entropy(e);
    match pair {
        None => None,
        Some(p) => Some(p.public.to_bytes())
    }
}