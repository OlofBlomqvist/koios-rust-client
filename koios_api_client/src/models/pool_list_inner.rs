




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolListInner {
    /// Bech32 representation of pool ID
    #[serde(rename = "pool_id_bech32", default, skip_serializing_if = "Option::is_none")]
    pub pool_id_bech32: Option<String>,
    /// Pool ticker
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

impl PoolListInner {
    pub fn new() -> PoolListInner {
        PoolListInner {
            pool_id_bech32: None,
            ticker: None,
        }
    }
}


