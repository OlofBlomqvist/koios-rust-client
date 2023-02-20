




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountRewardsInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::AccountRewardsInnerRewardsInner>>,
}

impl AccountRewardsInner {
    pub fn new() -> AccountRewardsInner {
        AccountRewardsInner {
            stake_address: None,
            rewards: None,
        }
    }
}


