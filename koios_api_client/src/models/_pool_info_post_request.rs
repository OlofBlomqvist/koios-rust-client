




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolInfoPostRequest {
    /// Array of Cardano pool IDs (bech32 format)
    #[serde(rename = "_pool_bech32_ids")]
    pub _pool_bech32_ids: Vec<String>,
}

impl PoolInfoPostRequest {
    pub fn new(_pool_bech32_ids: Vec<String>) -> PoolInfoPostRequest {
        PoolInfoPostRequest {
            _pool_bech32_ids,
        }
    }
}


