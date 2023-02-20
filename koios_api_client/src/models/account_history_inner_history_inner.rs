




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountHistoryInnerHistoryInner {
    /// Bech32 representation of pool ID
    #[serde(rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// Epoch number
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i32>,
    /// Active stake amount (in lovelaces)
    #[serde(rename = "active_stake", skip_serializing_if = "Option::is_none")]
    pub active_stake: Option<String>,
}

impl AccountHistoryInnerHistoryInner {
    pub fn new() -> AccountHistoryInnerHistoryInner {
        AccountHistoryInnerHistoryInner {
            pool_id: None,
            epoch_no: None,
            active_stake: None,
        }
    }
}


