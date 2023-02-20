




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlocksInner {
    /// Hash of the block
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Epoch number of the block
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i32>,
    /// Absolute slot number of the block
    #[serde(rename = "abs_slot", skip_serializing_if = "Option::is_none")]
    pub abs_slot: Option<i32>,
    /// Slot number of the block in epoch
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i32>,
    /// Block height
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i32>,
    /// Block size in bytes
    #[serde(rename = "block_size", skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    /// UNIX timestamp of the block
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i32>,
    /// Number of transactions in the block
    #[serde(rename = "tx_count", skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<i32>,
    /// VRF key of the block producer
    #[serde(rename = "vrf_key", skip_serializing_if = "Option::is_none")]
    pub vrf_key: Option<String>,
    /// Pool ID in bech32 format (null for pre-Shelley blocks)
    #[serde(rename = "pool", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pool: Option<Option<String>>,
    /// Counter value of the operational certificate used to create this block
    #[serde(rename = "op_cert_counter", skip_serializing_if = "Option::is_none")]
    pub op_cert_counter: Option<i32>,
    #[serde(rename = "proto_major", skip_serializing_if = "Option::is_none")]
    pub proto_major: Option<i128>,
    #[serde(rename = "proto_minor", skip_serializing_if = "Option::is_none")]
    pub proto_minor: Option<i128>,
}

impl BlocksInner {
    pub fn new() -> BlocksInner {
        BlocksInner {
            hash: None,
            epoch_no: None,
            abs_slot: None,
            epoch_slot: None,
            block_height: None,
            block_size: None,
            block_time: None,
            tx_count: None,
            vrf_key: None,
            pool: None,
            op_cert_counter: None,
            proto_major: None,
            proto_minor: None,
        }
    }
}


