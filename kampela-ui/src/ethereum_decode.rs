#[cfg(not(feature="std"))]
use alloc::{string::String, vec::Vec, format, borrow::ToOwned};
use alloy_dyn_abi::TypedData;
use eth_sign_request_parser::eip712::{Eip712DomainWithSaltString, TypedDataWithSaltString};

#[cfg(feature="std")]
use std::{string::String, vec::Vec, format, borrow::ToOwned};
use core::{fmt, fmt::Write};

use alloy_eips::{
    eip2718,
    eip2930::AccessList,
};
use alloy_rlp::Decodable;
use alloy_consensus::{TxEip1559, TxEip2930, TxEip4844, TxEip7702, TxLegacy};
use alloy_primitives::{Address, ChainId, FixedBytes};
use uuid::Uuid;

use alloy_primitives::hex::ToHexExt;

#[derive(Clone)]
pub enum SignDataType {
    EthTransactionData = 1,
    EthTypedData = 2,
    EthRawBytes = 3,
    EthTypedTransaction = 4
}

impl fmt::Display for SignDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SignDataType::EthTransactionData => write!(f, "eth-transaction-data"),
            SignDataType::EthTypedData => write!(f, "eth-typed-data"),
            SignDataType::EthRawBytes => write!(f, "eth-raw-bytes"),
            SignDataType::EthTypedTransaction => write!(f, "eth-typed-transaction")
        }
    }
}
#[derive(Clone)]
pub struct EthSignRequest{
    pub request_id: Option<Uuid>,
    pub sign_data: Vec<u8>,
    pub data_type: SignDataType,
    pub chain_id: ChainId,
    pub derivation_path: String,
    pub source_fingerprint: [u8;4],
    pub address: Option<Address>,
    pub origin: Option<String>,
}

impl fmt::Display for EthSignRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(request_id) = &self.request_id {
            writeln!(f, "request id: {}", request_id)?;
        };
        writeln!(f, "sign data: 0x{}", hex::encode(&self.sign_data))?;
        writeln!(f, "data type: {}", &self.data_type)?;
        writeln!(f, "chain id: {}", &self.chain_id)?;
        writeln!(f, "derivation path: {}", &self.derivation_path)?;
        writeln!(f, "source fingerprint: 0x{}", hex::encode(&self.source_fingerprint))?;
        if let Some(address) = &self.address {
            writeln!(f, "address: {}", address)?;
        };
        if let Some(origin) = &self.origin {
            writeln!(f, "origin: {}", origin)?;
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseTransactionError {
    Rlp(alloy_rlp::Error),
    Fmt(fmt::Error),
    Serde(serde_json::Error),
    Abi(alloy_dyn_abi::Error),
    UnsupportedTxType,
    InputIsEmpty
}

fn parse_eip_2718(sign_data: &[u8]) -> Result<String, ParseTransactionError> {
    let mut out = String::new();
    let mut buffer = &mut &sign_data[1..];
    match sign_data.first() {
        Some(&eip2718::EIP2930_TX_TYPE_ID) => {
            let tx = TxEip2930::decode(&mut buffer)
                .or_else(|e| Err(ParseTransactionError::Rlp(e)))?;
            write!(&mut out, "{}", DisplayableTxEip2930(&tx))
                .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
        },
        Some(&eip2718::EIP1559_TX_TYPE_ID) => {
            let tx = TxEip1559::decode(&mut buffer)
                .or_else(|e| Err(ParseTransactionError::Rlp(e)))?;
            write!(&mut out, "{}", DisplayableTxEip1559(&tx))
                .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
        },
        Some(&eip2718::EIP4844_TX_TYPE_ID) => {
            let tx = TxEip4844::decode(&mut buffer)
                .or_else(|e| Err(ParseTransactionError::Rlp(e)))?;
            write!(&mut out, "{}", DisplayableTxEip4844(&tx))
                .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
        },
        Some(&eip2718::EIP7702_TX_TYPE_ID) => {
            let tx = TxEip7702::decode(&mut buffer)
                .or_else(|e| Err(ParseTransactionError::Rlp(e)))?;
            write!(&mut out, "{}", DisplayableTxEip7702(&tx))
                .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
        },
        Some(_) => return Err(ParseTransactionError::UnsupportedTxType),
        None => return Err(ParseTransactionError::InputIsEmpty)
    };
    Ok(out)
}

struct DisplayableTxEip2930<'a>(&'a TxEip2930);

impl<'a> fmt::Display for DisplayableTxEip2930<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "chain id: {}", self.0.chain_id)?;
        writeln!(f, "nonce: {}", self.0.nonce)?;
        writeln!(f, "gas_price: {}", self.0.gas_price)?;
        writeln!(f, "gas limit: {}", self.0.gas_limit)?;
        writeln!(f, "to: 0x{}", self.0.to.to().map(|a| format!("{}", a)).unwrap_or("".to_owned()))?;
        writeln!(f, "value: {}", self.0.value)?;
        writeln!(f, "data: {}", self.0.input)?;
        writeln!(f, "access list: {}", DisplayableAccessList(&self.0.access_list))?;
        Ok(())
    }
}

struct DisplayableTxEip1559<'a>(&'a TxEip1559);

impl<'a> fmt::Display for DisplayableTxEip1559<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "chain id: {}", self.0.chain_id)?;
        writeln!(f, "nonce: {}", self.0.nonce)?;
        writeln!(f, "max priority fee per gas: {}", self.0.max_priority_fee_per_gas)?;
        writeln!(f, "max fee per gas: {}", self.0.max_fee_per_gas)?;
        writeln!(f, "gas limit: {}", self.0.gas_limit)?;
        writeln!(f, "destination: {}", self.0.to.to().map(|a| format!("{}", a)).unwrap_or("".to_owned()))?;
        writeln!(f, "value: {}", self.0.value)?;
        writeln!(f, "data: {}", self.0.input)?;
        writeln!(f, "access list: {}", DisplayableAccessList(&self.0.access_list))?;
        Ok(())
    }
}

struct DisplayableTxEip4844<'a>(&'a TxEip4844);

impl<'a> fmt::Display for DisplayableTxEip4844<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "chain id: {}", self.0.chain_id)?;
        writeln!(f, "nonce: {}", self.0.nonce)?;
        writeln!(f, "max priority fee per gas: {}", self.0.max_priority_fee_per_gas)?;
        writeln!(f, "max fee per gas: {}", self.0.max_fee_per_gas)?;
        writeln!(f, "gas limit: {}", self.0.gas_limit)?;
        writeln!(f, "to: {}", self.0.to)?;
        writeln!(f, "value: {}", self.0.value)?;
        writeln!(f, "data: {}", self.0.input)?;
        writeln!(f, "access list: {}", DisplayableAccessList(&self.0.access_list))?;
        writeln!(f, "max fee per blob gas: {}", self.0.max_fee_per_blob_gas)?;
        writeln!(f, "blob versioned hashes: {}", DisplayableBlobVersionedHashes(&self.0.blob_versioned_hashes))?;
        Ok(())
    }
}

struct DisplayableTxEip7702<'a>(&'a TxEip7702);

impl<'a> fmt::Display for DisplayableTxEip7702<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "chain id: {}", self.0.chain_id)?;
        writeln!(f, "nonce: {}", self.0.nonce)?;
        writeln!(f, "max priority fee per gas: {}", self.0.max_priority_fee_per_gas)?;
        writeln!(f, "max fee per gas: {}", self.0.max_fee_per_gas)?;
        writeln!(f, "gas limit: {}", self.0.gas_limit)?;
        writeln!(f, "destination: {}", self.0.to)?;
        writeln!(f, "value: {}", self.0.value)?;
        writeln!(f, "data: {}", self.0.input)?;
        writeln!(f, "access list: {}", DisplayableAccessList(&self.0.access_list))?;
        Ok(())
    }
}

struct DisplayableTxLegacy<'a>(&'a TxLegacy);

impl<'a> fmt::Display for DisplayableTxLegacy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "nonce: {}", self.0.nonce)?;
        writeln!(f, "gas price: {}", self.0.gas_price)?;
        writeln!(f, "gas limit: {}", self.0.gas_limit)?;
        writeln!(f, "to: {}", self.0.to.to().map(|a| format!("{}", a)).unwrap_or("".to_owned()))?;
        writeln!(f, "value: {}", self.0.value)?;
        writeln!(f, "data: {}", self.0.input)?;
        Ok(())
    }
}

struct DisplayableAccessList<'a>(&'a AccessList);

impl<'a> fmt::Display for DisplayableAccessList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut bracket_line_break = false;
        for list_item in self.0.0.iter() {
            write!(f, "\n\taddress: {:}", list_item.address)?;
            write!(f, " storage keys: [")?;
            let intend = list_item.storage_keys.len() == 1;
            for storage_key in list_item.storage_keys.iter() {
                if intend { write!(f, "\n\t\t")? };
                write!(f, "{},", storage_key)?;
            }
            write!(f, "]")?;
            bracket_line_break = true;
        }
        if bracket_line_break {
            write!(f, "\n")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

struct DisplayableBlobVersionedHashes<'a>(&'a Vec<FixedBytes<32>>);

impl<'a> fmt::Display for DisplayableBlobVersionedHashes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut bracket_line_break = false;
        for (i, versioned_hash) in self.0.iter().enumerate() {
            write!(f, "\n\t{}: 0x{:}", i, hex::encode(versioned_hash.0))?;
            bracket_line_break = true;
        }
        if bracket_line_break {
            write!(f, "\n")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

fn decode_message(sign_data: &[u8]) -> Result<String, ParseTransactionError> {
    let mut out = String::new();
    if let Ok(message) = String::from_utf8(sign_data.to_vec()) {
        writeln!(&mut out, "message: {}", message)
            .map_err(|e| ParseTransactionError::Fmt(e))?;
    } else {
        writeln!(&mut out, "data: 0x{}", hex::encode(sign_data))
            .map_err(|e| ParseTransactionError::Fmt(e))?;
    }

    Ok(out)
}

fn decode_legacy(sign_data: &[u8]) -> Result<String, ParseTransactionError> {
    let mut out = String::new();
    let mut buffer = sign_data;
    let tx = TxLegacy::decode(&mut buffer)
        .or_else(|e| Err(ParseTransactionError::Rlp(e)))?;
    write!(&mut out, "{}", DisplayableTxLegacy(&tx))
        .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;

    Ok(out)
}

fn decode_eip_712(sign_data: &[u8]) -> Result<String, ParseTransactionError> {
    
    let mut out = String::new();
    let json: TypedDataWithSaltString = serde_json::from_slice(sign_data)
        .or_else(|e| Err(ParseTransactionError::Serde(e)))?;

    writeln!(&mut out, "Primary type: {}", json.primary_type)
        .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
    writeln!(&mut out, "{:#}", DisplayableEip712Domain(&json.domain))
        .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
    writeln!(&mut out, "message: {:#}", json.message)
        .or_else(|e| Err(ParseTransactionError::Fmt(e)))?;
    let typed_data = TypedData::from(json);
    // check type validity
    typed_data.encode_type().or_else(|e| Err(ParseTransactionError::Abi(e)))?;
    typed_data.encode_data().or_else(|e| Err(ParseTransactionError::Abi(e)))?;

    Ok(out)
}

pub fn decode_ethereum_signing_payload(eth_sign_request: EthSignRequest) -> Result<String, ParseTransactionError>{
    match eth_sign_request.data_type {
        // "eth-transaction-data"
        SignDataType::EthTransactionData => decode_legacy(&eth_sign_request.sign_data),
        // "eth-typed-data"
        SignDataType::EthTypedData => decode_eip_712(&eth_sign_request.sign_data),
        // "eth-raw-bytes"
        SignDataType::EthRawBytes => decode_message(&eth_sign_request.sign_data),
        //"eth-typed-transaction"
        SignDataType::EthTypedTransaction => parse_eip_2718(&eth_sign_request.sign_data),
    }
}

struct DisplayableEip712Domain<'a>(&'a Eip712DomainWithSaltString);

impl<'a> fmt::Display for DisplayableEip712Domain<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Eip712Domain {{")?;

        if let Some(ref name) = self.0.base.name {
            writeln!(f, "  name: \"{name}\",")?;
        }
        if let Some(ref version) = self.0.base.version {
            writeln!(f, "  version: \"{version}\",")?;
        }
        if let Some(ref chain_id) = self.0.base.chain_id {
            writeln!(f, "  chain_id: {chain_id},")?;
        }
        if let Some(ref verifying_contract) = self.0.base.verifying_contract {
            writeln!(f, "  verifying_contract: {verifying_contract:?},")?;
        }
        if let Some(ref salt_string) = self.0.salt_string {
            writeln!(f, "  salt: {},", salt_string)?;
        } else if let Some(ref salt ) = self.0.base.salt {
            writeln!(f, "  salt: 0x{},", salt.encode_hex())?;
        }

        write!(f, "}}")
    }
}

