




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountHistoryInner {
    /// Cardano staking address (reward account) in bech32 format
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<crate::models::AccountHistoryInnerHistoryInner>>,
}

impl AccountHistoryInner {
    pub fn new() -> AccountHistoryInner {
        AccountHistoryInner {
            stake_address: None,
            history: None,
        }
    }
}


