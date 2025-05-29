use alloc::{string::String, vec::Vec};
use kampela_ui::platform::{PinCode, Platform};
use substrate_crypto_light::ecdsa::{Public, ChainCode};
use kampela_system::{
    devices::{
        flash::{read_encoded_entropy, store_encoded_entopy},
        psram::{psram_decode_call, psram_decode_extension, read_from_psram, PsramAccess},
        se_aes_gcm::{decode_entropy, encode_entropy, Protected},
        se_rng
    },
    psram_mnemonic::PsramWordList,
};

use crate::nfc::NfcTransactionPsramAccess;

pub struct Hardware {
    pin: PinCode,
    protected: Option<Protected>,
    address: Option<[u8; 76]>,
    transaction_psram_access: Option<NfcTransactionPsramAccess>,
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
        }
    }
}

impl Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type AsWordList = PsramWordList;

    type NfcTransaction = NfcTransactionPsramAccess;
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

    fn store_entropy(&mut self, e: &[u8]) {
        self.protected = if e.len() != 0 {
            let protected = encode_entropy(e);
            store_encoded_entopy(&protected);
            Some(protected)
        } else {
            None
        }
    }

    fn read_entropy(&mut self) {
        self.protected = read_encoded_entropy();
    }

    fn public(&self) -> Option<Public> {
        self.pair().map(|p| p.public().unwrap())
    }

    fn entropy(&self) -> Option<Vec<u8>> {
        self.protected.as_ref().map(|p| decode_entropy(p))
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_transaction(&mut self, transaction: Self::NfcTransaction) {
        self.transaction_psram_access = Some(transaction);
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
            .sign(&data_to_sign)
            .unwrap();

        let mut signature_with_id: [u8; 65] = [1; 65];
        signature_with_id[1..].copy_from_slice(&signature.0);
        let signature_with_id_bytes = hex::encode(signature_with_id)
            .into_bytes()
            .try_into()
            .expect("static length");

        signature_with_id_bytes
    }

    fn address(&mut self) -> &[u8; 76] {
        if let Some(ref a) = self.address {
            a
        } else {
            panic!("qr generation failed");
        }
    }

}
