




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolInfoInner {
    /// Pool ID (bech32 format)
    #[serde(rename = "pool_id_bech32", skip_serializing_if = "Option::is_none")]
    pub pool_id_bech32: Option<String>,
    /// Pool ID (Hex format)
    #[serde(rename = "pool_id_hex", skip_serializing_if = "Option::is_none")]
    pub pool_id_hex: Option<String>,
    #[serde(rename = "active_epoch_no", skip_serializing_if = "Option::is_none")]
    pub active_epoch_no: Option<i128>,
    /// Pool VRF key hash
    #[serde(rename = "vrf_key_hash", skip_serializing_if = "Option::is_none")]
    pub vrf_key_hash: Option<String>,
    /// Margin (decimal format)
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f32>,
    /// Pool fixed cost per epoch
    #[serde(rename = "fixed_cost", skip_serializing_if = "Option::is_none")]
    pub fixed_cost: Option<String>,
    /// Pool pledge in lovelace
    #[serde(rename = "pledge", skip_serializing_if = "Option::is_none")]
    pub pledge: Option<String>,
    /// Pool reward address
    #[serde(rename = "reward_addr", skip_serializing_if = "Option::is_none")]
    pub reward_addr: Option<String>,
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(rename = "relays", skip_serializing_if = "Option::is_none")]
    pub relays: Option<Vec<serde_json::value::Value>>,
    /// Pool metadata URL
    #[serde(rename = "meta_url", default, skip_serializing_if = "Option::is_none")]
    pub meta_url: Option<String>,
    /// Pool metadata hash
    #[serde(rename = "meta_hash", default, skip_serializing_if = "Option::is_none")]
    pub meta_hash: Option<String>,
    #[serde(rename = "meta_json", default, skip_serializing_if = "Option::is_none")]
    pub meta_json: Option<Box<serde_json::value::Value>>,
    /// Pool status
    #[serde(rename = "pool_status", skip_serializing_if = "Option::is_none")]
    pub pool_status: Option<PoolStatus>,
    /// Announced retiring epoch (nullable)
    #[serde(rename = "retiring_epoch", default, skip_serializing_if = "Option::is_none")]
    pub retiring_epoch: Option<i128>,
    /// Pool latest operational certificate hash
    #[serde(rename = "op_cert", default, skip_serializing_if = "Option::is_none")]
    pub op_cert: Option<String>,
    /// Pool latest operational certificate counter value
    #[serde(rename = "op_cert_counter", default, skip_serializing_if = "Option::is_none")]
    pub op_cert_counter: Option<i128>,
    /// Pool active stake (will be null post epoch transition until dbsync calculation is complete)
    #[serde(rename = "active_stake", default, skip_serializing_if = "Option::is_none")]
    pub active_stake: Option<String>,
    /// Pool relative active stake share
    #[serde(rename = "sigma", default, skip_serializing_if = "Option::is_none")]
    pub sigma: Option<f32>,
    /// Total pool blocks on chain
    #[serde(rename = "block_count", default, skip_serializing_if = "Option::is_none")]
    pub block_count: Option<i128>,
    /// Summary of account balance for all pool owner's
    #[serde(rename = "live_pledge", default, skip_serializing_if = "Option::is_none")]
    pub live_pledge: Option<String>,
    /// Pool live stake
    #[serde(rename = "live_stake", default, skip_serializing_if = "Option::is_none")]
    pub live_stake: Option<String>,
    /// Pool live delegator count
    #[serde(rename = "live_delegators", skip_serializing_if = "Option::is_none")]
    pub live_delegators: Option<i128>,
    /// Pool live saturation (decimal format)
    #[serde(rename = "live_saturation", default, skip_serializing_if = "Option::is_none")]
    pub live_saturation: Option<f32>,
}

impl PoolInfoInner {
    pub fn new() -> PoolInfoInner {
        PoolInfoInner {
            pool_id_bech32: None,
            pool_id_hex: None,
            active_epoch_no: None,
            vrf_key_hash: None,
            margin: None,
            fixed_cost: None,
            pledge: None,
            reward_addr: None,
            owners: None,
            relays: None,
            meta_url: None,
            meta_hash: None,
            meta_json: None,
            pool_status: None,
            retiring_epoch: None,
            op_cert: None,
            op_cert_counter: None,
            active_stake: None,
            sigma: None,
            block_count: None,
            live_pledge: None,
            live_stake: None,
            live_delegators: None,
            live_saturation: None,
        }
    }
}

/// Pool status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoolStatus {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "retiring")]
    Retiring,
    #[serde(rename = "retired")]
    Retired,
}

impl Default for PoolStatus {
    fn default() -> PoolStatus {
        Self::Registered
    }
}

