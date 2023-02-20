




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockInfoPostRequest {
    #[serde(rename = "_block_hashes")]
    pub _block_hashes: Vec<String>,
}

impl BlockInfoPostRequest {
    pub fn new(_block_hashes: Vec<String>) -> BlockInfoPostRequest {
        BlockInfoPostRequest {
            _block_hashes,
        }
    }
}


