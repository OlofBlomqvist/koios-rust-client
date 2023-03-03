




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EpochInfoInner {
    /// Epoch number
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    /// Total output value across all transactions in epoch
    #[serde(rename = "out_sum", skip_serializing_if = "Option::is_none")]
    pub out_sum: Option<String>,
    /// Total fees incurred by transactions in epoch
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<String>,
    /// Number of transactions submitted in epoch
    #[serde(rename = "tx_count", skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<i128>,
    /// Number of blocks created in epoch
    #[serde(rename = "blk_count", skip_serializing_if = "Option::is_none")]
    pub blk_count: Option<i128>,
    /// UNIX timestamp of the epoch start
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i128>,
    /// UNIX timestamp of the epoch end
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i128>,
    /// UNIX timestamp of the epoch's first block
    #[serde(rename = "first_block_time", skip_serializing_if = "Option::is_none")]
    pub first_block_time: Option<i128>,
    /// UNIX timestamp of the epoch's last block
    #[serde(rename = "last_block_time", skip_serializing_if = "Option::is_none")]
    pub last_block_time: Option<i128>,
    /// Total active stake in epoch stake snapshot (null for pre-Shelley epochs)
    #[serde(rename = "active_stake", default, skip_serializing_if = "Option::is_none")]
    pub active_stake: Option<String>,
    /// Total rewards earned in epoch (null for pre-Shelley epochs)
    #[serde(rename = "total_rewards", default, skip_serializing_if = "Option::is_none")]
    pub total_rewards: Option<String>,
    /// Average block reward for epoch (null for pre-Shelley epochs)
    #[serde(rename = "avg_blk_reward", default, skip_serializing_if = "Option::is_none")]
    pub avg_blk_reward: Option<String>,
}

impl EpochInfoInner {
    pub fn new() -> EpochInfoInner {
        EpochInfoInner {
            epoch_no: None,
            out_sum: None,
            fees: None,
            tx_count: None,
            blk_count: None,
            start_time: None,
            end_time: None,
            first_block_time: None,
            last_block_time: None,
            active_stake: None,
            total_rewards: None,
            avg_blk_reward: None,
        }
    }
}


