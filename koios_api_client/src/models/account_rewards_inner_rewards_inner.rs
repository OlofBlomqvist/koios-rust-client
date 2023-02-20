




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountRewardsInnerRewardsInner {
    #[serde(rename = "earned_epoch", skip_serializing_if = "Option::is_none")]
    pub earned_epoch: Option<i128>,
    #[serde(rename = "spendable_epoch", skip_serializing_if = "Option::is_none")]
    pub spendable_epoch: Option<i128>,
    /// Amount of rewards earned (in lovelace)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The source of the rewards
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
}

impl AccountRewardsInnerRewardsInner {
    pub fn new() -> AccountRewardsInnerRewardsInner {
        AccountRewardsInnerRewardsInner {
            earned_epoch: None,
            spendable_epoch: None,
            amount: None,
            r#type: None,
            pool_id: None,
        }
    }
}

/// The source of the rewards
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "leader")]
    Leader,
    #[serde(rename = "treasury")]
    Treasury,
    #[serde(rename = "reserves")]
    Reserves,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Member
    }
}

