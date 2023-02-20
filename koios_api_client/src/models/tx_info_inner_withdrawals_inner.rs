




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerWithdrawalsInner {
    /// Withdrawal amount (in lovelaces)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// A Cardano staking address (reward account, bech32 encoded)
    #[serde(rename = "stake_addr", skip_serializing_if = "Option::is_none")]
    pub stake_addr: Option<String>,
}

impl TxInfoInnerWithdrawalsInner {
    pub fn new() -> TxInfoInnerWithdrawalsInner {
        TxInfoInnerWithdrawalsInner {
            amount: None,
            stake_addr: None,
        }
    }
}


