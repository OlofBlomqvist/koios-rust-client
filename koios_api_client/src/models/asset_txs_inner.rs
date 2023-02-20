




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetTxsInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i128>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
}

impl AssetTxsInner {
    pub fn new() -> AssetTxsInner {
        AssetTxsInner {
            tx_hash: None,
            epoch_no: None,
            block_height: None,
            block_time: None,
        }
    }
}


