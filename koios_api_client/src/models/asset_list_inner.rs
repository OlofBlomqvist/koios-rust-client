




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetListInner {
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

impl AssetListInner {
    pub fn new() -> AssetListInner {
        AssetListInner {
            policy_id: None,
            asset_name: None,
            fingerprint: None,
        }
    }
}


