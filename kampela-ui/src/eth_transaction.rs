//! Ethereum transaction types and helpers.
//!
//! Uses raw 32-byte private keys directly (no BIP39/BIP32 mnemonic derivation).

use crate::error::KampelaError;
#[cfg(not(feature = "std"))]
use alloc::{string::String, string::ToString, vec::Vec};
use alloy_consensus::{SignableTransaction, TxEip1559};
use alloy_primitives::{keccak256, Address, FixedBytes, Signature, TxKind, U256};
use clear_signing::clear_call::ClearCallContext;
use clear_signing::display::Display;
use clear_signing::fields::ClearCall;
use clear_signing::registry::Registry;
use clear_signing::resolver::Message;
use clear_signing::sol::SolFunction;
use clear_signing_format::{format_clear_call, Contract, MetadataProvider, NativeToken, Token};
use libsecp256k1::{
    Message as SecpMessage, PublicKey, RecoveryId, SecretKey, Signature as SecpSignature,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::{string::String, string::ToString, vec::Vec};

use crate::eth_registry_data::{
    contract_list, native_token, token_list, well_known_contract_addresses, well_known_displays,
    well_known_token_addresses,
};
/// Ethereum transaction with clear-signing display specifications.
///
/// Uses alloy's canonical TxEip1559 type for transaction fields,
/// plus Kampela-specific clear-signing displays.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthTransaction {
    pub tx: TxEip1559,
    pub displays: Vec<Display>,
}

/// Creates a SigningKey directly from a 32-byte private key (entropy).
///
/// No BIP39/BIP32 derivation - the entropy IS the private key.
pub fn signing_key_from_entropy(entropy: &[u8]) -> Result<SecretKey, KampelaError> {
    if entropy.len() != 32 {
        return Err(KampelaError::KeyGeneration);
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(entropy);
    SecretKey::parse(&key).map_err(|_| KampelaError::KeyGeneration)
}

/// Derives Ethereum address from a 32-byte private key.
///
/// Address is the last 20 bytes of keccak256(uncompressed_public_key).
pub fn derive_eth_address(entropy: &[u8]) -> Result<Address, KampelaError> {
    let signing_key = signing_key_from_entropy(entropy)?;
    let pubkey = PublicKey::from_secret_key(&signing_key);
    let pubkey_bytes = pubkey.serialize();

    if pubkey_bytes.len() < 65 {
        return Err(KampelaError::KeyGeneration);
    }

    // Hash the public key (skip first byte which is 0x04 for uncompressed)
    let hash = keccak256(&pubkey_bytes[1..]);

    // Take last 20 bytes as address
    let mut address_bytes = [0u8; 20];
    address_bytes.copy_from_slice(&hash.as_slice()[12..]);

    Ok(Address::from_slice(&address_bytes))
}

/// Signs an EIP-1559 transaction and returns the RLP-encoded signed transaction.
pub fn sign_eip1559_transaction(
    tx: &EthTransaction,
    entropy: &[u8],
) -> Result<Vec<u8>, KampelaError> {
    let signing_key = signing_key_from_entropy(entropy)?;

    // Use TxEip1559 directly (no conversion needed)
    let hash = tx.tx.signature_hash();

    let message =
        SecpMessage::parse_slice(hash.as_slice()).map_err(|_| KampelaError::SigningFailed)?;
    let (sig, recid): (SecpSignature, RecoveryId) = libsecp256k1::sign(&message, &signing_key);
    let sig_bytes = sig.serialize();
    let r = U256::try_from_be_slice(&sig_bytes[0..32]).ok_or(KampelaError::SigningFailed)?;
    let s = U256::try_from_be_slice(&sig_bytes[32..64]).ok_or(KampelaError::SigningFailed)?;
    let y_parity = (recid.serialize() & 1) == 1;
    let signature = Signature::new(r, s, y_parity);

    // Clone tx.tx to convert into signed transaction
    let signed = tx.tx.clone().into_signed(signature);

    let mut raw = Vec::with_capacity(signed.eip2718_encoded_length());
    signed.eip2718_encode(&mut raw);

    Ok(raw)
}

pub fn format_eth_transaction_display(
    tx: &EthTransaction,
    sender: Address,
) -> Result<String, String> {
    // Extract recipient address from TxKind
    let to = match tx.tx.to {
        TxKind::Call(addr) => addr,
        TxKind::Create => return Err("Deploy transaction not supported".to_string()),
    };

    // Use input bytes directly (already Bytes type)
    let data = tx.tx.input.clone();

    let message = Message::new(sender, to, tx.tx.value, data);
    let context = ClearCallContext::new(tx.displays.clone());
    let registry = StaticRegistry::new().map_err(|e| e.to_string())?;

    let clear_call: ClearCall = context
        .parse_clear_call(message, &registry, 0)
        .map_err(|e| e.to_string())?;

    let provider = StaticMetadataProvider::new(sender);
    Ok(format_clear_call(&clear_call, &provider, 0, false, None))
}

struct StaticMetadataProvider {
    tokens: Vec<Token>,
    contracts: Vec<Contract>,
    native_token: NativeToken,
    user_address: Address,
}

impl StaticMetadataProvider {
    fn new(user_address: Address) -> Self {
        let tokens = token_list().tokens;
        let contracts = contract_list().contracts;
        let native_token = native_token();
        Self {
            tokens,
            contracts,
            native_token,
            user_address,
        }
    }
}

impl MetadataProvider for StaticMetadataProvider {
    fn get_token(&self, address: Address) -> Option<Token> {
        self.tokens.iter().find(|t| t.address == address).cloned()
    }

    fn get_contract(&self, address: Address) -> Option<Contract> {
        self.contracts
            .iter()
            .find(|c| c.address == address)
            .cloned()
            .or_else(|| {
                self.get_token(address).map(|t| Contract {
                    chain_id: t.chain_id,
                    address: t.address,
                    name: t.name.clone(),
                })
            })
    }

    fn get_native_token(&self) -> NativeToken {
        self.native_token.clone()
    }

    fn get_address_name(&self, address: Address) -> Option<String> {
        // Check if this is the user's address first
        if address == self.user_address {
            return Some("User".to_string());
        }
        // Otherwise, look up in contract list
        self.get_contract(address).map(|c| c.name)
    }
}

struct StaticRegistry {
    well_known_displays: Vec<Display>,
    well_known_contracts: Vec<Address>,
    well_known_tokens: Vec<Address>,
}

impl StaticRegistry {
    fn new() -> Result<Self, &'static str> {
        Ok(Self {
            well_known_displays: well_known_displays(),
            well_known_contracts: well_known_contract_addresses(),
            well_known_tokens: well_known_token_addresses(),
        })
    }
}

impl Registry for StaticRegistry {
    fn is_well_known_contract(&self, address: &Address) -> bool {
        self.well_known_contracts.contains(address) || self.well_known_tokens.contains(address)
    }

    fn is_well_known_token(&self, address: &Address) -> bool {
        self.well_known_tokens.contains(address)
    }

    fn get_well_known_display(
        &self,
        _address: &Address,
        selector: &FixedBytes<4>,
    ) -> Option<Display> {
        self.well_known_displays
            .iter()
            .find(|display| {
                SolFunction::parse(&display.abi)
                    .map(|fun| fun.selector() == *selector)
                    .unwrap_or(false)
            })
            .cloned()
    }
}
