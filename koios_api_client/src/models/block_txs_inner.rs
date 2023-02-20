




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockTxsInner {
    #[serde(rename = "block_hash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(rename = "tx_hashes", skip_serializing_if = "Option::is_none")]
    pub tx_hashes: Option<Vec<String>>,
}

impl BlockTxsInner {
    pub fn new() -> BlockTxsInner {
        BlockTxsInner {
            block_hash: None,
            tx_hashes: None,
        }
    }
}


