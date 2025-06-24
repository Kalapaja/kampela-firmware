use core::str::FromStr;

use alloc::{borrow::ToOwned, format, string::{String, ToString}, vec::Vec};
use bitcoin::{bip32::DerivationPath, hashes::{sha256d, Hash, HashEngine, Hmac}, secp256k1::Message};
use kampela_ui::platform::{PinCode, Platform, ErrorTransaction};
use substrate_crypto_light::{common::BIG_SEED_LEN, sr25519::Public};
use kampela_system::{
    devices::{
        flash::{read_encoded_entropy, store_encoded_entopy},
        psram::{psram_decode_call, psram_decode_extension, read_from_psram, PsramAccess},
        se_aes_gcm::{decode_seed, encode_seed, Protected},
        se_rng
    },
    psram_mnemonic::PsramWordList,
};
use tiny_keccak::{Hasher, Keccak};

use crate::nfc::{NfcEthSignRequestPsramAccess, NfcTransactionPsramAccess};

pub struct Hardware {
    pin: PinCode,
    protected: Option<Protected>,
    address: Option<[u8; 76]>,
    transaction_psram_access: Option<NfcTransactionPsramAccess>,
    eth_sign_request_psram_access: Option<NfcEthSignRequestPsramAccess>,
}

impl Hardware {
    pub fn new() -> Self {
        let protected = None;
        let pin_set = false; // TODO query storage
        let pin = [0; 4];
        Self {
            pin,
            protected,
            address: None,
            transaction_psram_access: None,
            eth_sign_request_psram_access: None,
        }
    }
}

impl Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type AsWordList = PsramWordList;

    type NfcTransaction = NfcTransactionPsramAccess;
    type NfcEthSignRequest = NfcEthSignRequestPsramAccess;

    fn get_wordlist() -> Self::AsWordList {
        PsramWordList
    }

    fn rng<'b>(_: &'b mut ()) -> Self::Rng<'static> {
        se_rng::SeRng{}
    }

    fn pin(&self) -> &PinCode {
        &self.pin
    }

    fn pin_mut(&mut self) -> &mut PinCode {
        &mut self.pin
    }

    fn store_seed(&mut self, e: &[u8]) {
        let seed = Self::from_entropy_to_eth_seed(e);
        self.protected = if e.len() != 0 {
            let protected = encode_seed(&seed);
            store_encoded_entopy(&protected);
            Some(protected)
        } else {
            None
        }
    }

    fn read_seed(&mut self) -> bool {
        self.protected = read_encoded_entropy();
        self.protected.is_some()
    }

    fn public(&self) -> Option<Public> {
        self.pair().map(|p| p.public())
    }

    fn seed(&self) -> Option<Vec<u8>> {
        self.protected.as_ref().map(|p| decode_seed(p))
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, transaction: Self::NfcTransaction) {
        self.transaction_psram_access = Some(transaction);
    }

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest) {
        self.eth_sign_request_psram_access = Some(sign_request);
    }


    fn call(&mut self) -> Option<String> {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => return None
        };

        let (decoded_call, specs, spec_name) = psram_decode_call(
            &transaction_psram_access.call_psram_access,
            &transaction_psram_access.metadata_psram_access,
        );

        let carded = decoded_call.card(0, &specs, &spec_name);
        let call = carded
            .into_iter()
            .map(|card| card.show())
            .collect::<Vec<String>>()
            .join("\n");

        Some(call)
    }

    fn extensions(&mut self) -> Option<String> {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => return None
        };
        
        let (decoded_extension, specs, spec_name) = psram_decode_extension(
            &transaction_psram_access.extension_psram_access,
            &transaction_psram_access.metadata_psram_access,
            &transaction_psram_access.genesis_hash_bytes_psram_access
        );

        let mut carded = Vec::new();
        for ext in decoded_extension.iter() {
            let addition_set = ext.card(0, true, &specs, &spec_name);
            if !addition_set.is_empty() {
                carded.extend_from_slice(&addition_set)
            }
        }
        let extensions = carded
            .into_iter()
            .map(|card| card.show())
            .collect::<Vec<String>>()
            .join("\n");

        Some(extensions)
    }

    fn ethereum(&mut self) -> Option<String> {
        let eth_sign_request_psram_access = match self.eth_sign_request_psram_access {
            Some(ref a) => a,
            None => return None
        };
        
        let request_id= read_from_psram(&eth_sign_request_psram_access.request_id);
        let sign_data = read_from_psram(&eth_sign_request_psram_access.sign_data);
        let data_type: u8 = read_from_psram(&eth_sign_request_psram_access.data_type)[0];
        let data_type: &str = match data_type {
            1 => "eth-transaction-data",
            2 => "eth-typed-data",
            3 => "eth-raw-bytes",
            4 => "eth-typed-transaction",
            _ => "unknown_transaction"
        };
        let chain_id: [u8; 8] = read_from_psram(&eth_sign_request_psram_access.chain_id).try_into().unwrap();
        let chain_id = u64::from_le_bytes(chain_id);
        let derivation_path = read_from_psram(&eth_sign_request_psram_access.derivation_path);
        let derivation_path = String::from_utf8(derivation_path).unwrap();

        let address = read_from_psram(&eth_sign_request_psram_access.address);
        let origin = read_from_psram(&eth_sign_request_psram_access.origin);
        let origin = String::from_utf8(origin).unwrap();

        let mut out = "request id: ".to_owned();
        out.push_str(&hex::encode(request_id));
        out.push_str("\nsign data: ");
        out.push_str(&hex::encode(sign_data));
        out.push_str("\n");
        out.push_str(data_type);
        out.push_str("\nchain id: ");
        out.push_str(&format!("{:}", chain_id));
        out.push_str("\nderivation path: ");
        out.push_str(&derivation_path);
        out.push_str("\naddress: 0x");
        out.push_str(&hex::encode(address));
        out.push_str("\norigin: ");
        out.push_str(&origin);
        Some(out)
    }
    // heavy function, should be called when blocking not crucial
    fn check_eth_transaction(&self) -> Result<(), ErrorTransaction> {
        let eth_sign_request_psram_access = match self.eth_sign_request_psram_access {
            Some(ref a) => a,
            None => return Ok(())
        };
        let secp = &Hardware::secp(&mut ());
        let xpriv = self.xpriv().expect("xpriv should be stored");
        let derivation_path = read_from_psram(&eth_sign_request_psram_access.derivation_path);
        let derivation_path = String::from_utf8(derivation_path).expect("siltti should encode derivation path correctly");
        let path = DerivationPath::from_str(&derivation_path).expect("siltti should encode derivation path correctly");

        let child_xpriv = xpriv.derive_priv(secp, &path).expect("path validity checked");
        let uncompressed = child_xpriv.to_keypair(secp).public_key().serialize_uncompressed();
        let mut  keccak256 = tiny_keccak::Keccak::v256();
        keccak256.update(&uncompressed[1..]);
        let mut hash = [0u8; 32];
        keccak256.finalize(&mut hash);
        let eth_address = &hash[12..]; // Last 20 bytes
    
        let address = read_from_psram(&eth_sign_request_psram_access.address);

        if eth_address != &address {
            return Err(ErrorTransaction::AddressUnmatch)
        }
        let source_fingerprint: [u8; 4] = read_from_psram(&eth_sign_request_psram_access.source_fingerprint).try_into().unwrap();
        let source_fingerprint = u32::from_le_bytes(source_fingerprint);

        if source_fingerprint != u32::from_be_bytes(xpriv.fingerprint(secp).to_bytes()) {
            return Err(ErrorTransaction::SourceFingerprintUnmatch)
        }

        Ok(())

    }

    fn signature(&mut self) -> [u8; 130] {
        let transaction_psram_access = match self.transaction_psram_access {
            Some(ref a) => a,
            None => panic!("qr generation failed")
        };
        
        let data_to_sign_psram_access = PsramAccess {
            start_address: transaction_psram_access.call_psram_access.start_address,
            total_len:
                transaction_psram_access.call_psram_access.total_len
                + &transaction_psram_access.extension_psram_access.total_len
        };
        let data_to_sign = read_from_psram(&data_to_sign_psram_access);

        let signature = self.pair()
            .expect("entropy should be stored at this point")
            .sign_external_rng(&data_to_sign, &mut Self::rng(&mut ()));

        let mut signature_with_id: [u8; 65] = [1; 65];
        signature_with_id[1..].copy_from_slice(&signature.0);
        let signature_with_id_bytes = hex::encode(signature_with_id)
            .into_bytes()
            .try_into()
            .expect("static length");

        signature_with_id_bytes
    }

    fn eth_signature(&mut self) -> ([u8; 16], [u8; 65]) {
        let eth_sign_request_psram_access = match self.eth_sign_request_psram_access {
            Some(ref a) => a,
            None => panic!("qr generation failed")
        };
        
        let path_string = String::from_utf8(read_from_psram(&eth_sign_request_psram_access.derivation_path)).unwrap();
        let path = DerivationPath::from_str(&path_string).unwrap();
        let sign_data = read_from_psram(&eth_sign_request_psram_access.sign_data);
        let data_type: u8 = read_from_psram(&eth_sign_request_psram_access.data_type)[0];
        let data_to_sign = match data_type {
            // "eth-transaction-data"
            1 => prepare_legacy(sign_data),
            // "eth-typed-data"
            2 => prepare_eip_712(sign_data),
            // "eth-raw-bytes"
            3 => prepare_eip_191(sign_data),
            //"eth-typed-transaction"
            4 => prepare_eip_2718(sign_data),
            _ => panic!("eth unsupported transaction")
        };
        let mut engine = Keccak::v256();
        engine.update(&data_to_sign);
        let mut hashed = [0u8; 32];
        engine.finalize(&mut hashed);
        let msg = Message::from_digest_slice(&hashed).unwrap();

        let secp = &Self::secp(&mut ());
        let sk = self.xpriv()
            .expect("entropy should be stored at this point")
            .derive_priv(secp, &path).unwrap()
            .to_keypair(secp).secret_key();
        let signature = secp.sign_ecdsa_recoverable(&msg, &sk);
        let (recover_id, sign_bytes) = signature.serialize_compact();
        
        // Ethereum expects `v` as 27 or 28
        let mut v = match data_type {
            1 | 2 | 3 => 27,
            4 => 0,
            _ => unreachable!("eth unsupported transaction")
        };
        v += recover_id.to_i32() as u8;
        let mut out = [0u8; 65];
        out[0..64].copy_from_slice(&sign_bytes); // r (32) + s (32)
        out[64] = v;
        
        let request_id: [u8; 16]= read_from_psram(&eth_sign_request_psram_access.request_id).try_into().expect("Size should be checked on reception");
        (request_id, out)
    }

    fn address(&mut self) -> &[u8; 76] {
        if let Some(ref a) = self.address {
            a
        } else {
            panic!("qr generation failed");
        }
    }
}

fn prepare_eip_191(sign_data: Vec<u8>) -> Vec<u8> {
    let mut message = Vec::new();
    let prefix = format!("\x19Ethereum Signed Message:\n{}", sign_data.len());
    message.extend_from_slice(prefix.as_bytes());
    message.extend_from_slice(&sign_data);
    message
}

fn prepare_legacy(sign_data: Vec<u8>) -> Vec<u8> {
    sign_data
}

fn prepare_eip_712(sign_data: Vec<u8>) -> Vec<u8> {
    sign_data
}

fn prepare_eip_2718(sign_data: Vec<u8>) -> Vec<u8> {
    sign_data
}