




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TotalsInner {
    /// Epoch number
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    /// Circulating UTxOs for given epoch (in lovelaces)
    #[serde(rename = "circulation", skip_serializing_if = "Option::is_none")]
    pub circulation: Option<String>,
    /// Funds in treasury for given epoch (in lovelaces)
    #[serde(rename = "treasury", skip_serializing_if = "Option::is_none")]
    pub treasury: Option<String>,
    /// Rewards accumulated as of given epoch (in lovelaces)
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<String>,
    /// Total Active Supply (sum of treasury funds, rewards, UTxOs, deposits and fees) for given epoch (in lovelaces)
    #[serde(rename = "supply", skip_serializing_if = "Option::is_none")]
    pub supply: Option<String>,
    /// Total Reserves yet to be unlocked on chain
    #[serde(rename = "reserves", skip_serializing_if = "Option::is_none")]
    pub reserves: Option<String>,
}

impl TotalsInner {
    pub fn new() -> TotalsInner {
        TotalsInner {
            epoch_no: None,
            circulation: None,
            treasury: None,
            reward: None,
            supply: None,
            reserves: None,
        }
    }
}


