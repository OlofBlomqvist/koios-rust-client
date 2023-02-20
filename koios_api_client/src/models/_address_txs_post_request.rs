




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressTxsPostRequest {
    /// Array of Cardano payment address(es) in bech32 format
    #[serde(rename = "_addresses")]
    pub _addresses: Vec<String>,
    /// Only fetch information after specific block height
    #[serde(rename = "_after_block_height", skip_serializing_if = "Option::is_none")]
    pub _after_block_height: Option<i32>,
}

impl AddressTxsPostRequest {
    pub fn new(_addresses: Vec<String>) -> AddressTxsPostRequest {
        AddressTxsPostRequest {
            _addresses,
            _after_block_height: None,
        }
    }
}


