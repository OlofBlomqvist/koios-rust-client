




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountUpdatesInnerUpdatesInner {
    /// Type of certificate submitted
    #[serde(rename = "action_type", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionType>,
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i128>,
    #[serde(rename = "absolute_slot", skip_serializing_if = "Option::is_none")]
    pub absolute_slot: Option<i128>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
}

impl AccountUpdatesInnerUpdatesInner {
    pub fn new() -> AccountUpdatesInnerUpdatesInner {
        AccountUpdatesInnerUpdatesInner {
            action_type: None,
            tx_hash: None,
            epoch_no: None,
            epoch_slot: None,
            absolute_slot: None,
            block_time: None,
        }
    }
}

/// Type of certificate submitted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "registration")]
    Registration,
    #[serde(rename = "delegation")]
    Delegation,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deregistration")]
    Deregistration,
}

impl Default for ActionType {
    fn default() -> ActionType {
        Self::Registration
    }
}

