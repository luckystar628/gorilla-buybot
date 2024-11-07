use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenInfo {  
    pub address: String,
    pub circulating_market_cap: Option<String>,
    pub decimals: String,
    pub exchange_rate: Option<String>,
    pub holders: String,
    pub icon_url: Option<String>,
    pub name: String,
    pub symbol: String,
    pub total_supply: String,
    pub volume_24h: Option<String>,
}