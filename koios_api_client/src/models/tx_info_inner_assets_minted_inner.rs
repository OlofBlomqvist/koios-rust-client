




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerAssetsMintedInner {
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Quantity of minted assets (negative on burn)
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}

impl TxInfoInnerAssetsMintedInner {
    pub fn new() -> TxInfoInnerAssetsMintedInner {
        TxInfoInnerAssetsMintedInner {
            policy_id: None,
            asset_name: None,
            fingerprint: None,
            quantity: None,
        }
    }
}


