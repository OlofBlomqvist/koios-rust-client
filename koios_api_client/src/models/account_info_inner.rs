




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountInfoInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    /// Stake address status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "delegated_pool", skip_serializing_if = "Option::is_none")]
    pub delegated_pool: Option<String>,
    /// Total balance of the account including UTxO, rewards and MIRs (in lovelace)
    #[serde(rename = "total_balance", skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<String>,
    /// Total UTxO balance of the account
    #[serde(rename = "utxo", skip_serializing_if = "Option::is_none")]
    pub utxo: Option<String>,
    /// Total rewards earned by the account
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<String>,
    /// Total rewards withdrawn by the account
    #[serde(rename = "withdrawals", skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<String>,
    /// Total rewards available for withdawal
    #[serde(rename = "rewards_available", skip_serializing_if = "Option::is_none")]
    pub rewards_available: Option<String>,
    /// Total reserves MIR value of the account
    #[serde(rename = "reserves", skip_serializing_if = "Option::is_none")]
    pub reserves: Option<String>,
    /// Total treasury MIR value of the account
    #[serde(rename = "treasury", skip_serializing_if = "Option::is_none")]
    pub treasury: Option<String>,
}

impl AccountInfoInner {
    pub fn new() -> AccountInfoInner {
        AccountInfoInner {
            stake_address: None,
            status: None,
            delegated_pool: None,
            total_balance: None,
            utxo: None,
            rewards: None,
            withdrawals: None,
            rewards_available: None,
            reserves: None,
            treasury: None,
        }
    }
}

/// Stake address status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "not registered")]
    NotRegistered,
}

impl Default for Status {
    fn default() -> Status {
        Self::Registered
    }
}

