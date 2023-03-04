




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerOutputsInnerAssetListInner {
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Quantity of assets on the UTxO
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i128>,
}

impl TxInfoInnerOutputsInnerAssetListInner {
    pub fn new() -> TxInfoInnerOutputsInnerAssetListInner {
        TxInfoInnerOutputsInnerAssetListInner {
            policy_id: None,
            decimals: None,
            asset_name: None,
            fingerprint: None,
            quantity: None,
        }
    }
}


