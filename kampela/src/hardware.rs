use kampela_ui::{
    ethereum_decode::EthSignRequest,
    platform::{PinCode, Platform},
    messages::EthSignRequestDecodeError
};
//use substrate_crypto_light::sr25519::Public;
use kampela_system::{
    devices::{
        flash::{read_encoded_entropy, store_encoded_entopy},
        psram::{psram_decode_eth_sign_request, NfcEthSignRequestPsramAccess},
        se_aes_gcm::{decode_seed, encode_seed, Protected},
        se_rng
    },
    psram_mnemonic::PsramWordList,
};

pub struct Hardware {
    pin: PinCode,
    protected: Option<Protected>,
    address: Option<[u8; 76]>,
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
            eth_sign_request_psram_access: None,
        }
    }
}

impl Platform for Hardware {
    type HAL = ();
    type Rng<'c> = se_rng::SeRng;
    type AsWordList = PsramWordList;

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
/*
    fn public(&self) -> Option<Public> {
        self.pair().map(|p| p.public())
    }
*/
    fn seed(&self) -> Option<[u8; 64]> {
        self.protected.as_ref().map(|p| decode_seed(p)).map_or(None, |s| s.try_into().ok())
    }

    fn set_address(&mut self, addr: [u8; 76]) {
        self.address = Some(addr);
    }

    fn set_eth_sign_request(&mut self, sign_request: Self::NfcEthSignRequest) {
        self.eth_sign_request_psram_access = Some(sign_request);
    }

/*
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
*/
    fn eth_sign_request(&self) -> Result<EthSignRequest, EthSignRequestDecodeError> {
        let eth_sign_request_psram_access = match self.eth_sign_request_psram_access {
            Some(ref a) => a,
            None => return Err(EthSignRequestDecodeError::NoEthSignRequestDataStored)
        };
        psram_decode_eth_sign_request(eth_sign_request_psram_access)
    }
/*
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
*/
    fn address(&mut self) -> &[u8; 76] {
        if let Some(ref a) = self.address {
            a
        } else {
            panic!("qr generation failed");
        }
    }
}
