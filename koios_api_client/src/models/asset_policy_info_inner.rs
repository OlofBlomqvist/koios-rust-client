




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetPolicyInfoInner {
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "asset_name_ascii", skip_serializing_if = "Option::is_none")]
    pub asset_name_ascii: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "minting_tx_metadata", skip_serializing_if = "Option::is_none")]
    pub minting_tx_metadata: Option<serde_json::value::Value>,
    #[serde(rename = "token_registry_metadata", skip_serializing_if = "Option::is_none")]
    pub token_registry_metadata: Option<serde_json::value::Value>,
    #[serde(rename = "total_supply", skip_serializing_if = "Option::is_none")]
    pub total_supply: Option<String>,
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i128>,
}

impl AssetPolicyInfoInner {
    pub fn new() -> AssetPolicyInfoInner {
        AssetPolicyInfoInner {
            asset_name: None,
            asset_name_ascii: None,
            fingerprint: None,
            minting_tx_metadata: None,
            token_registry_metadata: None,
            total_supply: None,
            creation_time: None,
        }
    }
}


