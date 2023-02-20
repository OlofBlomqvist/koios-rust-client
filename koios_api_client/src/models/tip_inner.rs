




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TipInner {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "abs_slot", skip_serializing_if = "Option::is_none")]
    pub abs_slot: Option<i128>,
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i128>,
    #[serde(rename = "block_no", skip_serializing_if = "Option::is_none")]
    pub block_no: Option<i128>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
}

impl TipInner {
    pub fn new() -> TipInner {
        TipInner {
            hash: None,
            epoch_no: None,
            abs_slot: None,
            epoch_slot: None,
            block_no: None,
            block_time: None,
        }
    }
}


