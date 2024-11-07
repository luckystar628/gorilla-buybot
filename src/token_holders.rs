use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenHoldersData {
    pub items: Vec<TokenHolder>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenHolder {
    pub address: Address,
    pub token: TokenInfo,
    pub token_id: Option<String>,
    pub value: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Address {
    pub ens_domain_name: Option<String>,
    pub hash: String,
    pub implementation_address: Option<String>,
    pub implementation_name: Option<String>,
    pub implementations: Vec<String>,
    pub is_contract: bool,
    pub is_verified: bool,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub private_tags: Vec<String>,
    pub public_tags: Vec<String>,
    pub watchlist_names: Vec<String>,
}

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

// {
//     "items": [
//       {
//         "address": {
//           "ens_domain_name": null,
//           "hash": "0x99556e210123da382eDEd3c72AA8DCb605C3c435",
//           "implementation_address": null,
//           "implementation_name": null,
//           "implementations": [],
//           "is_contract": true,
//           "is_verified": true,
//           "metadata": null,
//           "name": "AlgebraPool",
//           "private_tags": [],
//           "public_tags": [],
//           "watchlist_names": []
//         },
//         "token": {
//           "address": "0x48b62137EdfA95a428D35C09E44256a739F6B557",
//           "circulating_market_cap": null,
//           "decimals": "18",
//           "exchange_rate": null,
//           "holders": "10030",
//           "icon_url": null,
//           "name": "Wrapped ApeCoin",
//           "symbol": "WAPE",
//           "total_supply": "11433851064038647957351649",
//           "type": "ERC-20",
//           "volume_24h": null
//         },
//         "token_id": null,
//         "value": "6503861090159114670615466"
//       },
//       ...
//     ]
//   }



        