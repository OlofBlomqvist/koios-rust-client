




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolDelegatorsInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    /// Current delegator live stake (in lovelace)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Epoch number in which the delegation becomes active
    #[serde(rename = "active_epoch_no", skip_serializing_if = "Option::is_none")]
    pub active_epoch_no: Option<i128>,
    /// Latest transaction hash used for delegation by the account
    #[serde(rename = "latest_delegation_hash", skip_serializing_if = "Option::is_none")]
    pub latest_delegation_hash: Option<String>,
}

impl PoolDelegatorsInner {
    pub fn new() -> PoolDelegatorsInner {
        PoolDelegatorsInner {
            stake_address: None,
            amount: None,
            active_epoch_no: None,
            latest_delegation_hash: None,
        }
    }
}


