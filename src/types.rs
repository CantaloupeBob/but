#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BroadcastJson {
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
    pub libraries: Vec<Library>,
    pub pending: Vec<Pending>,
    pub returns: Return,
    pub timestamp: usize,
    pub chain: usize,
    pub commit: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: String,
    #[serde(deserialize_with = "deserialize_transaction_type")]
    pub transaction_type: TransactionType,
    pub contract_name: Option<String>,
    pub contract_address: String,
    pub function: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub transaction: TransactionInner,
    pub additional_contracts: Vec<AdditionalContract>,
    pub is_fixed_gas_limit: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInner {
    pub from: String,
    pub gas: String,
    pub value: String,
    pub input: String,
    pub nonce: String,
    pub chain_id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum TransactionType {
    Create,
    Create2,
    Call,
}

// TODO: placeholder
#[derive(Deserialize, Debug)]
pub struct AdditionalContract {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub status: String, // 0x1, 0x0
    pub cumulative_gas_used: String,
    pub logs: Vec<Log>,
    pub logs_bloom: String,
    pub type_: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub block_hash: String,
    pub block_number: String,
    pub gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub to: Option<String>,
    pub contract_address: Option<String>,
    pub gas_used_for_l1: Option<String>,
    pub l1_block_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_hash: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub log_index: String,
    pub removed: bool,
}

// "<path>:<contract_name>:<address>"
pub type Library = String;

// TODO: placeholder
#[derive(Deserialize, Debug)]
pub struct Pending {}

// TODO: placeholder
#[derive(Deserialize, Debug)]
pub struct Return {}

fn deserialize_transaction_type<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "CREATE" => Ok(TransactionType::Create),
        "CREATE2" => Ok(TransactionType::Create2),
        "CALL" => Ok(TransactionType::Call),
        _ => Err(serde::de::Error::custom(format!(
            "Unknown transaction type: {}",
            s
        ))),
    }
}
