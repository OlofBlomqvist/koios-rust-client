




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolRelaysInner {
    #[serde(rename = "pool_id_bech32", skip_serializing_if = "Option::is_none")]
    pub pool_id_bech32: Option<String>,
    #[serde(rename = "relays", skip_serializing_if = "Option::is_none")]
    pub relays: Option<serde_json::value::Value>,
}

impl PoolRelaysInner {
    pub fn new() -> PoolRelaysInner {
        PoolRelaysInner {
            pool_id_bech32: None,
            relays: None,
        }
    }
}


