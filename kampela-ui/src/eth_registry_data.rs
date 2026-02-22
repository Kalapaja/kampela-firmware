#[cfg(not(feature="std"))]
use alloc::{string::ToString, vec, vec::Vec};
#[cfg(feature="std")]
use std::{string::ToString, vec, vec::Vec};

use alloy_primitives::Address;
use clear_signing::display::{Display, Entry, Field, Labels};
use clear_signing_format::{Contract, ContractList, NativeToken, Token, TokenList, Version};
use core::str::FromStr;

fn address(hex: &str) -> Address {
    Address::from_str(hex).expect("static address is valid")
}

pub fn native_token() -> NativeToken {
    NativeToken {
        name: "Ether".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18,
        logo_uri: Some(
            "https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/info/logo.png"
                .to_string(),
        ),
    }
}

pub fn token_list() -> TokenList {
    TokenList {
        schema: None,
        name: "Local Token List".to_string(),
        timestamp: "2024-01-01T00:00:00+00:00".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        tokens: vec![
            Token {
                chain_id: 31337,
                address: address("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
                name: "Ether".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
                logo_uri: Some(
                    "https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/info/logo.png"
                        .to_string(),
                ),
            },
            Token {
                chain_id: 31337,
                address: address("0x6B175474E89094C44Da98b954EedeAC495271d0F"),
                name: "Dai Stablecoin".to_string(),
                symbol: "DAI".to_string(),
                decimals: 18,
                logo_uri: Some(
                    "https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x6B175474E89094C44Da98b954EedeAC495271d0F/logo.png"
                        .to_string(),
                ),
            },
            Token {
                chain_id: 31337,
                address: address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
                name: "Wrapped Ether".to_string(),
                symbol: "WETH".to_string(),
                decimals: 18,
                logo_uri: Some(
                    "https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2/logo.png"
                        .to_string(),
                ),
            },
        ],
    }
}

pub fn contract_list() -> ContractList {
    ContractList {
        schema: None,
        name: "Local Contract List".to_string(),
        timestamp: "2026-01-03T12:10:37+01:00".to_string(),
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        contracts: vec![Contract {
            chain_id: 31337,
            address: address("0xd512108c249cC5ec5370491AD916Be31bb88Dad2"),
            name: "Clear Call Router".to_string(),
        }],
    }
}

pub fn well_known_token_addresses() -> Vec<Address> {
    vec![
        address("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
        address("0x6B175474E89094C44Da98b954EedeAC495271d0F"),
        address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
    ]
}

pub fn well_known_contract_addresses() -> Vec<Address> {
    vec![address("0xd512108c249cC5ec5370491AD916Be31bb88Dad2")]
}

pub fn well_known_displays() -> Vec<Display> {
    vec![
        display_approve(),
        display_transfer(),
        display_deposit(),
        display_withdraw(),
    ]
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
        checks: vec![],
        fields: vec![],
        params,
    }
}

fn display_approve() -> Display {
    Display {
        abi: "function approve(address spender, uint256 amount)".to_string(),
        title: "$labels.approve".to_string(),
        description: "$labels.approve_description".to_string(),
        fields: vec![
            field(
                "$labels.spender",
                "$labels.spender_description",
                "contract",
                vec![entry("value", "$locals.spender")],
            ),
            field(
                "$labels.amount",
                "$labels.amount_description",
                "tokenAmount",
                vec![entry("token", "$msg.to"), entry("amount", "$locals.amount")],
            ),
        ],
        labels: vec![labels(&[
            ("approve", "Approve Token Spending"),
            (
                "approve_description",
                "Allow a contract or address to spend your tokens on your behalf",
            ),
            ("spender", "Spender Address"),
            (
                "spender_description",
                "The contract or address that will be allowed to transfer your tokens",
            ),
            ("amount", "Spending Limit"),
            (
                "amount_description",
                "Maximum amount the spender can transfer from your balance",
            ),
        ])],
    }
}

fn display_transfer() -> Display {
    Display {
        abi: "function transfer(address to, uint256 amount)".to_string(),
        title: "$labels.transfer".to_string(),
        description: "$labels.transfer_description".to_string(),
        fields: vec![
            field(
                "$labels.to",
                "$labels.to_description",
                "address",
                vec![entry("value", "$locals.to")],
            ),
            field(
                "$labels.amount",
                "$labels.amount_description",
                "tokenAmount",
                vec![entry("token", "$msg.to"), entry("amount", "$locals.amount")],
            ),
        ],
        labels: vec![labels(&[
            ("transfer", "Transfer Tokens"),
            ("transfer_description", "Send tokens directly to another address"),
            ("to", "Recipient"),
            ("to_description", "The address that will receive the tokens"),
            ("amount", "Amount to Send"),
            (
                "amount_description",
                "Number of tokens to transfer to the recipient",
            ),
        ])],
    }
}

fn display_deposit() -> Display {
    Display {
        abi: "function deposit() payable".to_string(),
        title: "$labels.deposit".to_string(),
        description: "$labels.deposit_description".to_string(),
        fields: vec![
            field(
                "$labels.sending",
                "$labels.sending_description",
                "nativeAmount",
                vec![entry("amount", "$msg.value")],
            ),
            field(
                "$labels.receiving",
                "$labels.receiving_description",
                "tokenAmount",
                vec![entry("token", "$msg.to"), entry("amount", "$msg.value")],
            ),
        ],
        labels: vec![labels(&[
            ("deposit", "Wrap ETH to WETH"),
            (
                "deposit_description",
                "Convert your ETH into WETH (Wrapped Ether) tokens",
            ),
            ("sending", "You're Sending"),
            ("sending_description", "Amount of ETH you're depositing"),
            ("receiving", "You're Receiving"),
            (
                "receiving_description",
                "Amount of WETH tokens you'll receive (1:1 ratio)",
            ),
        ])],
    }
}

fn display_withdraw() -> Display {
    Display {
        abi: "function withdraw(uint256 wad)".to_string(),
        title: "$labels.withdraw".to_string(),
        description: "$labels.withdraw_description".to_string(),
        fields: vec![
            field(
                "$labels.sending",
                "$labels.sending_description",
                "tokenAmount",
                vec![entry("token", "$msg.to"), entry("amount", "$locals.wad")],
            ),
            field(
                "$labels.receiving",
                "$labels.receiving_description",
                "nativeAmount",
                vec![entry("amount", "$locals.wad")],
            ),
        ],
        labels: vec![labels(&[
            ("withdraw", "Unwrap WETH to ETH"),
            (
                "withdraw_description",
                "Convert your WETH (Wrapped Ether) back into native ETH",
            ),
            ("sending", "You're Sending"),
            (
                "sending_description",
                "Amount of WETH tokens you're unwrapping",
            ),
            ("receiving", "You're Receiving"),
            (
                "receiving_description",
                "Amount of ETH you'll receive (1:1 ratio)",
            ),
        ])],
    }
}
