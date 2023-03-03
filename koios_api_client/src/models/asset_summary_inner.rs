




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetSummaryInner {
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "asset_name", skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Total number of transactions including the given asset
    #[serde(rename = "total_transactions", skip_serializing_if = "Option::is_none")]
    pub total_transactions: Option<i128>,
    /// Total number of registered wallets holding the given asset
    #[serde(rename = "staked_wallets", skip_serializing_if = "Option::is_none")]
    pub staked_wallets: Option<i128>,
    /// Total number of payment addresses (not belonging to registered wallets) holding the given asset
    #[serde(rename = "unstaked_addresses", skip_serializing_if = "Option::is_none")]
    pub unstaked_addresses: Option<i128>,
}

impl AssetSummaryInner {
    pub fn new() -> AssetSummaryInner {
        AssetSummaryInner {
            policy_id: None,
            asset_name: None,
            fingerprint: None,
            total_transactions: None,
            staked_wallets: None,
            unstaked_addresses: None,
        }
    }
}


