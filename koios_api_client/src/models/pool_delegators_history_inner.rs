




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolDelegatorsHistoryInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Epoch number for the delegation history
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i32>,
}

impl PoolDelegatorsHistoryInner {
    pub fn new() -> PoolDelegatorsHistoryInner {
        PoolDelegatorsHistoryInner {
            stake_address: None,
            amount: None,
            epoch_no: None,
        }
    }
}


