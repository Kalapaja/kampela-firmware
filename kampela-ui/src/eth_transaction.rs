//! Ethereum transaction types.

#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use crate::platform::EthAddress;
use alloy_consensus::{SignableTransaction, TxEip1559};
use alloy_primitives::{keccak256, Address, Bytes, Signature, TxKind, U256};
use bip32::{DerivationPath, XPrv};
use bip39::{Language, Mnemonic};
use k256::ecdsa::SigningKey;

#[derive(Clone, Debug)]
pub struct Eip1559Transaction {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: u128,
    pub max_fee_per_gas: u128,
    pub gas_limit: u64,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
}

pub fn derive_eth_signing_key(entropy: &[u8]) -> Option<SigningKey> {
    let mnemonic = Mnemonic::from_entropy_in(Language::English, entropy).ok()?;
    let seed = mnemonic.to_seed("");
    let path: DerivationPath = "m/44'/60'/0'/0/0".parse().ok()?;
    let child_xprv = XPrv::derive_from_path(&seed, &path).ok()?;
    let private_key = child_xprv.private_key().to_bytes();
    SigningKey::from_bytes(&private_key).ok()
}

pub fn derive_eth_address(entropy: &[u8]) -> Option<EthAddress> {
    let signing_key = derive_eth_signing_key(entropy)?;
    let verifying_key = signing_key.verifying_key();
    let pubkey = verifying_key.to_encoded_point(false);
    let pubkey_bytes = pubkey.as_bytes();
    if pubkey_bytes.len() < 65 {
        return None;
    }
    let hash = keccak256(&pubkey_bytes[1..]);
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash.as_slice()[12..]);
    Some(address)
}

pub fn sign_eip1559_transaction(tx: &Eip1559Transaction, entropy: &[u8]) -> Option<Vec<u8>> {
    let signing_key = derive_eth_signing_key(entropy)?;
    let tx = build_alloy_tx(tx);
    let hash = tx.signature_hash();
    let (sig, recid) = signing_key.sign_prehash_recoverable(hash.as_slice()).ok()?;
    let signature = Signature::from((sig, recid));
    let signed = tx.into_signed(signature);
    let mut raw = Vec::with_capacity(signed.eip2718_encoded_length());
    signed.eip2718_encode(&mut raw);
    Some(raw)
}

fn build_alloy_tx(tx: &Eip1559Transaction) -> TxEip1559 {
    let to = match tx.to {
        Some(addr) => TxKind::Call(addr),
        None => TxKind::Create,
    };

    TxEip1559 {
        chain_id: tx.chain_id,
        nonce: tx.nonce,
        gas_limit: tx.gas_limit,
        max_fee_per_gas: tx.max_fee_per_gas,
        max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
        to,
        value: tx.value,
        access_list: Default::default(),
        input: tx.data.clone(),
    }
}
