




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetInfoInner {
    /// Asset Policy ID (hex)
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Asset Name (hex)
    #[serde(rename = "asset_name", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// Asset Name (ASCII)
    #[serde(rename = "asset_name_ascii", skip_serializing_if = "Option::is_none")]
    pub asset_name_ascii: Option<String>,
    /// The CIP14 fingerprint of the asset
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Hash of the first mint transaction
    #[serde(rename = "minting_tx_hash", skip_serializing_if = "Option::is_none")]
    pub minting_tx_hash: Option<String>,
    /// Count of total mint transactions
    #[serde(rename = "mint_cnt", skip_serializing_if = "Option::is_none")]
    pub mint_cnt: Option<i32>,
    /// Count of total burn transactions
    #[serde(rename = "burn_cnt", skip_serializing_if = "Option::is_none")]
    pub burn_cnt: Option<i32>,
    /// Latest minting transaction metadata (aligns with CIP-25)
    #[serde(rename = "minting_tx_metadata", skip_serializing_if = "Option::is_none")]
    pub minting_tx_metadata: Option<Box<serde_json::value::Value>>,
    #[serde(rename = "token_registry_metadata", default, skip_serializing_if = "Option::is_none")]
    pub token_registry_metadata: Option<Box<crate::models::AssetInfoInnerTokenRegistryMetadata>>,
    #[serde(rename = "total_supply", skip_serializing_if = "Option::is_none")]
    pub total_supply: Option<String>,
    /// UNIX timestamp of the first asset mint
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i32>,
}

impl AssetInfoInner {
    pub fn new() -> AssetInfoInner {
        AssetInfoInner {
            policy_id: None,
            asset_name: None,
            asset_name_ascii: None,
            fingerprint: None,
            minting_tx_hash: None,
            mint_cnt: None,
            burn_cnt: None,
            minting_tx_metadata: None,
            token_registry_metadata: None,
            total_supply: None,
            creation_time: None,
        }
    }
}


