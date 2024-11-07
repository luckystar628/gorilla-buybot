use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]    
pub struct TokenOverview {
    pub id: String,
    pub chain: String,
    pub name: String,
    pub symbol: String,
    pub display_symbol: Option<String>,
    pub optimized_symbol: Option<String>,
    pub decimals: u64,
    pub logo_url: String,
    pub protocol_id: String,
    pub price: f64,
    pub price_24h_change: f64,
    pub credit_score: f64,
    pub is_verified: bool,
    pub is_scam: bool,
    pub is_suspicious: bool,
    pub is_core: bool,
    pub is_wallet: bool,
    pub time_at: f64,
    pub low_credit_score: bool
}