




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetListInner {
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "decimals")]
    pub decimals: i128,
}

impl AssetListInner {
    pub fn new() -> AssetListInner {
        AssetListInner {
            policy_id: "".into(),
            asset_name: None,
            fingerprint: "".into(),
            decimals:0,
            quantity:"0".into()
        }
    }
}


