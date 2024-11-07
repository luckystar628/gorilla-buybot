use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TxInfo {
    pub timestamp: String,
    pub fee: Fee,
    pub gas_limit: String,
    pub block: u64,
    pub status: String,
    pub method: String,
    pub confirmations: u64,
    pub from: AddressInfo,
    pub to: AddressInfo,
    pub tx_burnt_fee: Option<String>,
    pub max_fee_per_gas: Option<String>,
    pub result: String,
    pub gas_price: String,
    pub priority_fee: Option<String>,
    pub base_fee_per_gas: Option<String>,
    // pub token_transfers: Vec<TokenTransferItem>,
    pub tx_types: Vec<String>,
    pub gas_used: String,
    // pub created_contract: Option<AddressInfo>,
    // pub position: u64,
    // pub nonce: u64,
    // pub has_error_in_internal_txs: bool,
    // pub actions: Vec<Action>,
    // pub decoded_input: DecodedInput,
    // pub token_transfers_overflow: bool,
    // pub raw_input: String,
    pub value: String,
    pub max_priority_fee_per_gas: Option<String>,
    // pub revert_reason: Option<String>,
    // pub confirmation_duration: Vec<u64>,
    // pub tx_tag: Option<String>,

}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Fee {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AddressInfo {
    pub hash: String,
    pub implementation_name: Option<String>,
    pub name: Option<String>,
    pub is_contract: bool,
}



