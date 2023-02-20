




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxMetadataInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// A JSON array containing details about metadata within transaction
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl TxMetadataInner {
    pub fn new() -> TxMetadataInner {
        TxMetadataInner {
            tx_hash: None,
            metadata: None,
        }
    }
}


