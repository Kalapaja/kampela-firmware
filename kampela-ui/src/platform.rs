//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec};
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1::SignOnly, NetworkKind};
use sha2::Sha512;
use hmac::Hmac;
use pbkdf2::pbkdf2;
#[cfg(feature="std")]
use std::{string::String, vec::Vec};

use rand::{CryptoRng, Rng};

use substrate_crypto_light::{common::{entropy_to_big_seed, BIG_SEED_LEN, HASH_256_LEN}, sr25519::{Pair, Public}};
use substrate_parser::{TransactionUnmarkedParsed, ShortSpecs};

use mnemonic_external::{AsWordList, WordSet};

pub type PinCode = [u8; 4];
const ENTROPY_LEN: usize = 32; //TODO: move to appropriate place

pub enum ErrorTransaction {
    AddressUnmatch,
    SourceFingerprintUnmatch,
}

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

    /// Transaction data or addresses for transaction data in psram
    type NfcEthSignRequest;

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

    /// Put seed in flash
    fn store_seed(&mut self, e: &[u8]);

    /// Read seed from flash
    fn read_seed(&mut self) -> bool;

    /// Getter for public address
    fn public(&self) -> Option<Public>;
    
    /// Getter for seed
    fn seed(&self) -> Option<Vec<u8>>;

    fn set_address(&mut self, addr: [u8; 76]);

    fn set_transaction(&mut self, transaction: Self::NfcTransaction);

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest);

    fn call(&mut self) -> Option<String>;

    fn extensions(&mut self) -> Option<String>;

    fn ethereum(&mut self) -> Option<String>;

    fn signature(&mut self) -> [u8; 130];

    fn eth_signature(&mut self) -> ([u8;16], [u8; 65]);

    fn address(&mut self) -> &[u8; 76];

    //----derivatives----

    fn check_eth_transaction(&self) -> Result<(), ErrorTransaction>;

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }

    fn pair(&self) -> Option<Pair> {
        let e = self.seed()?;  // not entropy, shall get pair from seed
        if e.is_empty() { None } else {
            Pair::from_entropy_and_pwd(&e, "").ok()
        }
    }

    fn xpriv(&self) -> Option<Xpriv> {
        Xpriv::new_master(NetworkKind::Test, &self.seed()?).ok()
    }

    fn secp(h: &mut Self::HAL) -> Secp256k1<SignOnly> {
        let mut secp = Secp256k1::<SignOnly>::gen_new();
        let mut seed = [0u8; 32];
        Self::rng(h).fill(&mut seed);
        secp.seeded_randomize(&seed);
        secp
    }

    fn from_entropy_to_eth_seed(entropy: &[u8]) -> [u8; 64] {
        let wordlist = Self::get_wordlist();
        let a = WordSet::from_entropy(entropy).unwrap();
        let mnemonic = a.to_phrase(&wordlist).unwrap();

        let password = "";
        let mut salt = String::with_capacity(8 + password.len());
        salt.push_str("mnemonic");
        salt.push_str(password);
    
        let mut seed = [0u8; BIG_SEED_LEN];
        pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut seed).unwrap();
        seed
    }

}

pub struct NfcTransaction {
    pub decoded_transaction: TransactionUnmarkedParsed,
    pub data_to_sign: Vec<u8>,
    pub specs: ShortSpecs,
    pub spec_name: String,
}
