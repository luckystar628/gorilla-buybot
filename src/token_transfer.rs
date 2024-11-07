use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenTransfer {
    pub items: Vec<TokenTransferItem>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenTransferItem {
    pub block_hash: String,
    pub from: AddressInfo,
    pub to: AddressInfo,
    pub token: TokenInfo,
    pub total: Total,
    pub log_index: String,
    pub method: String,
    pub timestamp: String,
    pub tx_hash: String,
    pub r#type: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub ens_domain_name: Option<String>,
    pub hash: String,
    pub implementation_address: Option<String>,
    pub implementation_name: Option<String>,
    pub is_contract: bool,
    pub is_verified: bool,
    pub name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Total {
    pub decimals: String,
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub circulating_market_cap: Option<String>,
    pub exchange_rate: Option<String>,
    pub holders: String,
    pub icon_url: Option<String>,
    pub name: String,
    pub symbol: String,
    pub decimals: String,
    pub total_supply: String,
    pub volume_24h: Option<String>,
}

// When deserializing, you'll need a custom implementation:
impl From<Vec<(String, AddressInfo, AddressInfo, TokenInfo, Total, String, String, String, String, String)>> for TokenTransfer {
    fn from(data: Vec<(String, AddressInfo, AddressInfo, TokenInfo, Total, String, String, String, String, String)>) -> Self {
        let items = data
            .into_iter()
            .map(|item| TokenTransferItem {
                block_hash: item.0,
                from: item.1,
                to: item.2,
                token: item.3,
                total: item.4,
                log_index: item.5,
                method: item.6,
                timestamp: item.7,
                tx_hash: item.8,
                r#type: item.9,
            })
            .collect();
        TokenTransfer { items }
    }
}




// {
//     "items": [
    //   {
    //     "block_hash": "0xb8ac245829f5989222357430cab7f22329ea8f1cecfee6ea7106c02903d37515",
    //     "from": {
    //       "ens_domain_name": null,
    //       "hash": "0x4DB3a1131c9f12fC9C2ca1f060d5b94D0dCc11de",
    //       "implementation_address": null,
    //       "implementation_name": null,
    //       "implementations": [],
    //       "is_contract": true,
    //       "is_verified": true,
    //       "metadata": null,
    //       "name": "UTB",
    //       "private_tags": [],
    //       "public_tags": [],
    //       "watchlist_names": []
    //     },
    //     "log_index": "6",
    //     "method": "0x7cd44734",
    //     "timestamp": "2024-11-06T15:41:14.000000Z",
    //     "to": {
    //       "ens_domain_name": null,
    //       "hash": "0x0000000000000000000000000000000000000000",
    //       "implementation_address": null,
    //       "implementation_name": null,
    //       "implementations": [],
    //       "is_contract": false,
    //       "is_verified": false,
    //       "metadata": null,
    //       "name": null,
    //       "private_tags": [],
    //       "public_tags": [],
    //       "watchlist_names": []
    //     },
    //     "token": {
    //       "address": "0x48b62137EdfA95a428D35C09E44256a739F6B557",
    //       "circulating_market_cap": null,
    //       "decimals": "18",
    //       "exchange_rate": null,
    //       "holders": "10039",
    //       "icon_url": null,
    //       "name": "Wrapped ApeCoin",
    //       "symbol": "WAPE",
    //       "total_supply": "11430907751224090057358708",
    //       "type": "ERC-20",
    //       "volume_24h": null
    //     },
    //     "total": {
    //       "decimals": "18",
    //       "value": "186942772000000000000"
    //     },
    //     "tx_hash": "0x6ec0abf0735974c746a9c092b73a10ce6e79bf3d8d18c10485046ffbd66bf273",
    //     "type": "token_burning"
    //   },
//       ...
//     ]
// }