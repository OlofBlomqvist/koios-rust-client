




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolMetadataPostRequest {
    /// Array of Cardano pool IDs (bech32 format)
    #[serde(rename = "_pool_bech32_ids", skip_serializing_if = "Option::is_none")]
    pub _pool_bech32_ids: Option<Vec<String>>,
}

impl PoolMetadataPostRequest {
    pub fn new() -> PoolMetadataPostRequest {
        PoolMetadataPostRequest {
            _pool_bech32_ids: None,
        }
    }
}


