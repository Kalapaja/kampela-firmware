use alloy_consensus::TxEip1559;
use alloy_primitives::{Address, Bytes, TxKind, U256};
use clear_signing::display::{Display, Entry, Field, Labels};
use core::str::FromStr;
use lazy_static::lazy_static;

use kampela_ui::eth_transaction::EthTransaction;

lazy_static! {
    static ref SAMPLE_ETH_TX: EthTransaction = EthTransaction {
        tx: TxEip1559 {
            chain_id: 1,
            nonce: 0,
            max_priority_fee_per_gas: 1_500_000_000,
            max_fee_per_gas: 50_000_000_000,
            gas_limit: 21_000,
            to: TxKind::Call(address_from_str("0xec5ab17cc35221cdf54eaeb0868ea82d4d75d9bf")),
            value: U256::from(0u64),
            access_list: Default::default(),
            input: Bytes::from(hex_to_bytes("0xb4a28e959e653efaceb1d170641a41fda59def0f499b671eaa2ffe332520816269e0bd000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000010438ed17390000000000000000000000000000000000000000000000056bc75e2d6310000000000000000000000000000000000000000000000000000000700501e120f1a900000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000056451bbcebbb1a764b52a7fb1e90ac07536dac5000000000000000000000000000000000000000000000000000000006966512600000000000000000000000000000000000000000000000000000000000000020000000000000000000000006b175474e89094c44da98b954eedeac495271d0f000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000")),
        },
        displays: vec![Display {
            address: address_from_str("0xec5ab17cc35221cdf54eaeb0868ea82d4d75d9bf"),
            abi: "function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] path, address to, uint256 deadline)".to_string(),
            title: "$labels.swap".to_string(),
            description: "$labels.swap_description".to_string(),
            fields: vec![
                field(
                    "$labels.sending",
                    "$labels.sending_description",
                    "tokenAmount",
                    vec![
                        entry("token", "$locals.path[0]"),
                        entry("amount", "$locals.amountIn"),
                    ],
                ),
                field(
                    "$labels.receiving_min",
                    "$labels.receiving_min_description",
                    "tokenAmount",
                    vec![
                        entry("token", "$locals.path[-1]"),
                        entry("amount", "$locals.amountOutMin"),
                    ],
                ),
                field(
                    "$labels.recipient",
                    "$labels.recipient_description",
                    "address",
                    vec![entry("value", "$locals.to")],
                ),
                field(
                    "$labels.deadline",
                    "$labels.deadline_description",
                    "datetime",
                    vec![entry("value", "$locals.deadline")],
                ),
            ],
            labels: vec![labels(&[
                ("swap", "Swap Tokens"),
                ("swap_description", "Exchange one token for another at the current market rate"),
                ("sending", "You're Sending"),
                ("sending_description", "Exact amount of tokens you're swapping"),
                ("receiving_min", "You're Receiving (minimum)"),
                ("receiving_min_description", "Minimum amount you'll receive - protects against price slippage"),
                ("recipient", "Recipient"),
                ("recipient_description", "Address that will receive the output tokens"),
                ("deadline", "Deadline"),
                ("deadline_description", "Transaction must complete before this time"),
            ])],
        }],
    };
}

pub fn sample_eth_transaction() -> EthTransaction {
    SAMPLE_ETH_TX.clone()
}

fn address_from_str(hex: &str) -> Address {
    Address::from_str(hex).expect("static address is valid")
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let trimmed = hex.strip_prefix("0x").unwrap_or(hex);
    hex::decode(trimmed).expect("static hex is valid")
}

fn entry(key: &str, value: &str) -> Entry {
    Entry {
        key: key.to_string(),
        value: value.to_string(),
    }
}

fn labels(items: &[(&str, &str)]) -> Labels {
    let items = items
        .iter()
        .map(|(key, value)| entry(key, value))
        .collect();
    Labels {
        locale: "en".to_string(),
        items,
    }
}

fn field(title: &str, description: &str, format: &str, params: Vec<Entry>) -> Field {
    Field {
        title: title.to_string(),
        description: description.to_string(),
        format: format.to_string(),
        checks: Vec::new(),
        params,
    }
}

/// Serializes the sample Ethereum transaction to postcard bytes for NFC transmission.
///
/// Wire format: [0x04 discriminator][postcard-encoded EthTransaction]
///
/// This demonstrates the expected NFC payload format that the device will receive.
pub fn serialize_sample_tx_to_postcard() -> Vec<u8> {
    let mut payload = vec![0x04]; // Discriminator byte for Ethereum transaction

    // Serialize the transaction using postcard
    let tx_bytes = postcard::to_allocvec(&*SAMPLE_ETH_TX)
        .expect("Failed to serialize sample transaction");

    payload.extend_from_slice(&tx_bytes);
    payload
}

/// Returns a reference to the sample Ethereum transaction for testing
pub fn sample_eth_tx() -> &'static EthTransaction {
    &SAMPLE_ETH_TX
}

/// Serializes a string test message for NFC transmission.
///
/// Wire format: [0x05 discriminator][UTF-8 string bytes]
///
/// This is used for testing the test message display feature.
/// Example: serialize_test_message("Hello, Kampela!")
pub fn serialize_test_message(text: &str) -> Vec<u8> {
    let mut data = vec![0x05]; // Discriminator byte for test message
    data.extend_from_slice(text.as_bytes());
    data
}
