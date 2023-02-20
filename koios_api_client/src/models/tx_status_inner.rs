




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxStatusInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Number of block confirmations
    #[serde(rename = "num_confirmations", default, skip_serializing_if = "Option::is_none")]
    pub num_confirmations: Option<i32>,
}

impl TxStatusInner {
    pub fn new() -> TxStatusInner {
        TxStatusInner {
            tx_hash: None,
            num_confirmations: None,
        }
    }
}


