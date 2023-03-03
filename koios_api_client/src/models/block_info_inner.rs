




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockInfoInner {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "abs_slot", skip_serializing_if = "Option::is_none")]
    pub abs_slot: Option<i128>,
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i128>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i128>,
    #[serde(rename = "block_size", skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i128>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
    #[serde(rename = "tx_count", skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<i128>,
    #[serde(rename = "vrf_key", skip_serializing_if = "Option::is_none")]
    pub vrf_key: Option<String>,
    /// Hash of the block producers' operational certificate
    #[serde(rename = "op_cert", skip_serializing_if = "Option::is_none")]
    pub op_cert: Option<String>,
    #[serde(rename = "op_cert_counter", skip_serializing_if = "Option::is_none")]
    pub op_cert_counter: Option<i128>,
    #[serde(rename = "pool", skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(rename = "proto_major", skip_serializing_if = "Option::is_none")]
    pub proto_major: Option<i128>,
    #[serde(rename = "proto_minor", skip_serializing_if = "Option::is_none")]
    pub proto_minor: Option<i128>,
    /// Total output of the block (in lovelace)
    #[serde(rename = "total_output", default, skip_serializing_if = "Option::is_none")]
    pub total_output: Option<String>,
    /// Total fees of the block (in lovelace)
    #[serde(rename = "total_fees", default, skip_serializing_if = "Option::is_none")]
    pub total_fees: Option<String>,
    /// Number of confirmations for the block
    #[serde(rename = "num_confirmations", skip_serializing_if = "Option::is_none")]
    pub num_confirmations: Option<i128>,
    /// Hash of the parent of this block
    #[serde(rename = "parent_hash", skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<String>,
    /// Hash of the child of this block (if present)
    #[serde(rename = "child_hash", skip_serializing_if = "Option::is_none")]
    pub child_hash: Option<String>,
}

impl BlockInfoInner {
    pub fn new() -> BlockInfoInner {
        BlockInfoInner {
            hash: None,
            epoch_no: None,
            abs_slot: None,
            epoch_slot: None,
            block_height: None,
            block_size: None,
            block_time: None,
            tx_count: None,
            vrf_key: None,
            op_cert: None,
            op_cert_counter: None,
            pool: None,
            proto_major: None,
            proto_minor: None,
            total_output: None,
            total_fees: None,
            num_confirmations: None,
            parent_hash: None,
            child_hash: None,
        }
    }
}


