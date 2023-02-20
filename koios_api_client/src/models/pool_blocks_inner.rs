




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolBlocksInner {
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i128>,
    #[serde(rename = "abs_slot", skip_serializing_if = "Option::is_none")]
    pub abs_slot: Option<i128>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i128>,
    #[serde(rename = "block_hash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
}

impl PoolBlocksInner {
    pub fn new() -> PoolBlocksInner {
        PoolBlocksInner {
            epoch_no: None,
            epoch_slot: None,
            abs_slot: None,
            block_height: None,
            block_hash: None,
            block_time: None,
        }
    }
}


