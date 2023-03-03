




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolUpdatesInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
    #[serde(rename = "pool_id_bech32", skip_serializing_if = "Option::is_none")]
    pub pool_id_bech32: Option<String>,
    #[serde(rename = "pool_id_hex", skip_serializing_if = "Option::is_none")]
    pub pool_id_hex: Option<String>,
    /// Epoch number in which the update becomes active
    #[serde(rename = "active_epoch_no", skip_serializing_if = "Option::is_none")]
    pub active_epoch_no: Option<i128>,
    #[serde(rename = "vrf_key_hash", skip_serializing_if = "Option::is_none")]
    pub vrf_key_hash: Option<String>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<i128>,
    #[serde(rename = "fixed_cost", skip_serializing_if = "Option::is_none")]
    pub fixed_cost: Option<String>,
    #[serde(rename = "pledge", skip_serializing_if = "Option::is_none")]
    pub pledge: Option<String>,
    #[serde(rename = "reward_addr", skip_serializing_if = "Option::is_none")]
    pub reward_addr: Option<String>,
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(rename = "relays", skip_serializing_if = "Option::is_none")]
    pub relays: Option<serde_json::value::Value>,
    #[serde(rename = "meta_url", skip_serializing_if = "Option::is_none")]
    pub meta_url: Option<String>,
    #[serde(rename = "meta_hash", skip_serializing_if = "Option::is_none")]
    pub meta_hash: Option<String>,
    #[serde(rename = "meta_json", skip_serializing_if = "Option::is_none")]
    pub meta_json: Option<serde_json::value::Value>,
    #[serde(rename = "pool_status", skip_serializing_if = "Option::is_none")]
    pub pool_status: Option<String>,
    #[serde(rename = "retiring_epoch", skip_serializing_if = "Option::is_none")]
    pub retiring_epoch: Option<i128>,
}

impl PoolUpdatesInner {
    pub fn new() -> PoolUpdatesInner {
        PoolUpdatesInner {
            tx_hash: None,
            block_time: None,
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
        }
    }
}


