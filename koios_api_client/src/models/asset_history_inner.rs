




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetHistoryInner {
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Array of all mint/burn transactions for an asset
    #[serde(rename = "minting_txs", default, skip_serializing_if = "Option::is_none")]
    pub minting_txs: Option<Vec<crate::models::AssetHistoryInnerMintingTxsInner>>,
}

impl AssetHistoryInner {
    pub fn new() -> AssetHistoryInner {
        AssetHistoryInner {
            policy_id: None,
            asset_name: None,
            fingerprint: None,
            minting_txs: None,
        }
    }
}


