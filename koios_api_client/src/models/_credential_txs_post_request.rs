




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CredentialTxsPostRequest {
    /// Array of Cardano payment credential(s) in hex format
    #[serde(rename = "_payment_credentials")]
    pub _payment_credentials: Vec<String>,
    /// Only fetch information after specific block height
    #[serde(rename = "_after_block_height", skip_serializing_if = "Option::is_none")]
    pub _after_block_height: Option<i32>,
}

impl CredentialTxsPostRequest {
    pub fn new(_payment_credentials: Vec<String>) -> CredentialTxsPostRequest {
        CredentialTxsPostRequest {
            _payment_credentials,
            _after_block_height: None,
        }
    }
}


