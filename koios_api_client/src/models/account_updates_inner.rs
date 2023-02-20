




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountUpdatesInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::AccountUpdatesInnerUpdatesInner>>,
}

impl AccountUpdatesInner {
    pub fn new() -> AccountUpdatesInner {
        AccountUpdatesInner {
            stake_address: None,
            updates: None,
        }
    }
}


