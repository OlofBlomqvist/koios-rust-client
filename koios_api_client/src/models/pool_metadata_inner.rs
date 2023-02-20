




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolMetadataInner {
    #[serde(rename = "pool_id_bech32", skip_serializing_if = "Option::is_none")]
    pub pool_id_bech32: Option<String>,
    #[serde(rename = "meta_url", skip_serializing_if = "Option::is_none")]
    pub meta_url: Option<String>,
    #[serde(rename = "meta_hash", skip_serializing_if = "Option::is_none")]
    pub meta_hash: Option<String>,
    #[serde(rename = "meta_json", skip_serializing_if = "Option::is_none")]
    pub meta_json: Option<serde_json::value::Value>,
}

impl PoolMetadataInner {
    pub fn new() -> PoolMetadataInner {
        PoolMetadataInner {
            pool_id_bech32: None,
            meta_url: None,
            meta_hash: None,
            meta_json: None,
        }
    }
}


