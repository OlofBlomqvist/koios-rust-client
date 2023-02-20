




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoPostRequest {
    /// Array of Cardano Transaction hashes
    #[serde(rename = "_tx_hashes")]
    pub _tx_hashes: Vec<String>,
}

impl TxInfoPostRequest {
    pub fn new(_tx_hashes: Vec<String>) -> TxInfoPostRequest {
        TxInfoPostRequest {
            _tx_hashes,
        }
    }
}


