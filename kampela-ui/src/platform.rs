//! Platform definitions

#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec, format};
use alloy_dyn_abi::TypedData;
use alloy_primitives::FixedBytes;
use eth_sign_request_parser::eip712::TypedDataWithSaltString;
use core::str::FromStr;
use bitcoin::{bip32::{DerivationPath, Xpriv}, key::Secp256k1, secp256k1::{Message, SignOnly}, NetworkKind};
use sha2::Sha512;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use tiny_keccak::{Keccak, Hasher};
#[cfg(feature="std")]
use std::{string::String, vec::Vec, format};

use rand::{CryptoRng, Rng};

//use substrate_crypto_light::{common::{BIG_SEED_LEN}, sr25519::{Pair, Public}};
//use substrate_parser::{TransactionUnmarkedParsed, ShortSpecs};


use mnemonic_external::{AsWordList, WordSet};

use crate::{ethereum_decode::{EthSignRequest, SignDataType}, messages::{ErrorTransaction, EthSignRequestDecodeError}};

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
    //fn public(&self) -> Option<Public>;
    
    /// Getter for seed
    fn seed(&self) -> Option<Vec<u8>>;

    fn set_address(&mut self, addr: [u8; 76]);

    fn set_transaction(&mut self, transaction: Self::NfcTransaction);

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest);

    //fn call(&mut self) -> Option<String>;

    //fn extensions(&mut self) -> Option<String>;

    fn eth_sign_request(&self) -> Result<EthSignRequest, EthSignRequestDecodeError>;

    //fn signature(&mut self) -> [u8; 130];

    fn address(&mut self) -> &[u8; 76];

    //----derivatives----

    // heavy function, should be called when blocking not crucial
    fn check_eth_transaction(&self, h: &mut Self::HAL) -> Result<(), ErrorTransaction> {
        let eth_sign_request_data = self.eth_sign_request()
            .map_err(|e| ErrorTransaction::SignReustDecodeError(e))?;

        let secp = &Self::secp(h);
        let xpriv = self.xpriv().expect("xpriv should be stored");

        let path = DerivationPath::from_str(&eth_sign_request_data.derivation_path)
            .or(Err(ErrorTransaction::SignReustDecodeError(EthSignRequestDecodeError::InvalidDerivationPathString)))?;

        let child_xpriv = xpriv.derive_priv(secp, &path)
            .or(Err(ErrorTransaction::SignReustDecodeError(EthSignRequestDecodeError::InvalidDerivationPathString)))?;
        let uncompressed = child_xpriv.to_keypair(secp).public_key().serialize_uncompressed();
        let mut  keccak256 = Keccak::v256();
        keccak256.update(&uncompressed[1..]);
        let mut hash = [0u8; 32];
        keccak256.finalize(&mut hash);
        let eth_address = &hash[12..]; // Last 20 bytes

        if let Some(a) = eth_sign_request_data.address {
            if eth_address != &a {
                return Err(ErrorTransaction::AddressUnmatch)
            }
        }

        if xpriv.fingerprint(secp).to_bytes() != eth_sign_request_data.source_fingerprint {
            return Err(ErrorTransaction::SourceFingerprintUnmatch)
        }

        Ok(())

    }

    fn generate_seed_entropy(h: &mut Self::HAL) -> [u8; ENTROPY_LEN] {
        let mut entropy: [u8; ENTROPY_LEN]= [0; ENTROPY_LEN];
        Self::rng(h).fill(&mut entropy);
        entropy
    }
/*
    fn pair(&self) -> Option<Pair> {
        let e = self.seed()?;  // not entropy, shall get pair from seed
        if e.is_empty() { None } else {
            Pair::from_entropy_and_pwd(&e, "").ok()
        }
    }
*/
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
    
        let mut seed = [0u8; 64];
        pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut seed).unwrap();
        seed
    }

    fn eth_signature(&mut self, h: &mut Self::HAL) -> ([u8; 16], [u8; 65]) {
        let eth_sign_request = self.eth_sign_request().expect("sign request checked");
        let data_to_sign: FixedBytes<32> = match eth_sign_request.data_type {

            SignDataType::EthTypedData => prepare_eip_712(eth_sign_request.sign_data),

            SignDataType::EthRawBytes => prepare_eip_191(eth_sign_request.sign_data),

            SignDataType::EthTypedTransaction |
            SignDataType::EthTransactionData => hash_sign_data(eth_sign_request.sign_data),
        };

        let msg = Message::from_digest_slice(&data_to_sign.0).unwrap();

        let secp = &Self::secp(h);
        let path = DerivationPath::from_str(&eth_sign_request.derivation_path).expect("derivation path checked");
        let sk = self.xpriv()
            .expect("entropy should be stored at this point")
            .derive_priv(secp, &path).unwrap()
            .to_keypair(secp).secret_key();
        let signature = secp.sign_ecdsa_recoverable(&msg, &sk);
        let (recover_id, sign_bytes) = signature.serialize_compact();
        
        // Ethereum expects `v` as 27 or 28
        let mut v = match eth_sign_request.data_type { 
            SignDataType::EthTypedTransaction => 0,
            _ => 27
        };
        v += recover_id.to_i32() as u8;
        let mut out = [0u8; 65];
        out[0..64].copy_from_slice(&sign_bytes); // r (32) + s (32)
        out[64] = v;
        
        let request_id: [u8; 16] = *eth_sign_request.request_id.unwrap().as_bytes();
        (request_id, out)
    }

}

fn prepare_eip_191(sign_data: Vec<u8>) -> FixedBytes<32> {
    let mut message = Vec::new();
    let prefix = format!("\x19Ethereum Signed Message:\n{}", sign_data.len());
    message.extend_from_slice(prefix.as_bytes());
    message.extend_from_slice(&sign_data);

    hash_sign_data(message)
}

fn prepare_eip_712(sign_data: Vec<u8>) -> FixedBytes<32> {
    let json: TypedDataWithSaltString = serde_json::from_slice(&sign_data)
        .expect("parsing checked");
    TypedData::from(json).eip712_signing_hash()
        .expect("types validity checked")
}
fn hash_sign_data(sign_data: Vec<u8>) -> FixedBytes<32> {
    let mut engine = Keccak::v256();
    engine.update(&sign_data);
    let mut hashed = FixedBytes::<32>::ZERO;
    engine.finalize(&mut hashed.0);
    hashed
}
/*
pub struct NfcTransaction {
    pub decoded_transaction: TransactionUnmarkedParsed,
    pub data_to_sign: Vec<u8>,
    pub specs: ShortSpecs,
    pub spec_name: String,
}
*/