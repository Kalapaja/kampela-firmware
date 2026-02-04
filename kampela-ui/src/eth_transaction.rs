//! Ethereum transaction types.

#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use alloy_primitives::B256;

use crate::platform::EthAddress;

#[derive(Clone, Debug)]
pub struct Eip1559Transaction {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: B256,
    pub max_fee_per_gas: B256,
    pub gas_limit: u64,
    pub to: Option<EthAddress>,
    pub value: B256,
    pub data: Vec<u8>,
}
