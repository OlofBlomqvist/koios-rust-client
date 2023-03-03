




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountRewardsPostRequest {
    /// Array of Cardano stake address(es) in bech32 format
    #[serde(rename = "_stake_addresses")]
    pub _stake_addresses: Vec<String>,
    /// Only fetch information for a specific epoch
    #[serde(rename = "_epoch_no", skip_serializing_if = "Option::is_none")]
    pub _epoch_no: Option<i128>,
}

impl AccountRewardsPostRequest {
    pub fn new(_stake_addresses: Vec<String>) -> AccountRewardsPostRequest {
        AccountRewardsPostRequest {
            _stake_addresses,
            _epoch_no: None,
        }
    }
}


