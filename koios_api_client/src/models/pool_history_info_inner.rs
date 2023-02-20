




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolHistoryInfoInner {
    /// Epoch for which the pool history data is shown
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i32>,
    /// Amount of delegated stake to this pool at the time of epoch snapshot (in lovelaces)
    #[serde(rename = "active_stake", skip_serializing_if = "Option::is_none")]
    pub active_stake: Option<String>,
    /// Active stake for the pool, expressed as a percentage of total active stake on network
    #[serde(rename = "active_stake_pct", skip_serializing_if = "Option::is_none")]
    pub active_stake_pct: Option<f32>,
    /// Saturation percentage of a pool at the time of snapshot (2 decimals)
    #[serde(rename = "saturation_pct", skip_serializing_if = "Option::is_none")]
    pub saturation_pct: Option<f32>,
    /// Number of blocks pool created in that epoch
    #[serde(rename = "block_cnt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub block_cnt: Option<Option<i32>>,
    /// Number of delegators to the pool for that epoch snapshot
    #[serde(rename = "delegator_cnt", skip_serializing_if = "Option::is_none")]
    pub delegator_cnt: Option<i32>,
    /// Margin (decimal format)
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f32>,
    /// Pool fixed cost per epoch (in lovelaces)
    #[serde(rename = "fixed_cost", skip_serializing_if = "Option::is_none")]
    pub fixed_cost: Option<String>,
    /// Total amount of fees earned by pool owners in that epoch (in lovelaces)
    #[serde(rename = "pool_fees", skip_serializing_if = "Option::is_none")]
    pub pool_fees: Option<String>,
    /// Total amount of rewards earned by delegators in that epoch (in lovelaces)
    #[serde(rename = "deleg_rewards", skip_serializing_if = "Option::is_none")]
    pub deleg_rewards: Option<String>,
    /// Annualized ROS (return on staking) for delegators for this epoch
    #[serde(rename = "epoch_ros", skip_serializing_if = "Option::is_none")]
    pub epoch_ros: Option<f32>,
}

impl PoolHistoryInfoInner {
    pub fn new() -> PoolHistoryInfoInner {
        PoolHistoryInfoInner {
            epoch_no: None,
            active_stake: None,
            active_stake_pct: None,
            saturation_pct: None,
            block_cnt: None,
            delegator_cnt: None,
            margin: None,
            fixed_cost: None,
            pool_fees: None,
            deleg_rewards: None,
            epoch_ros: None,
        }
    }
}


