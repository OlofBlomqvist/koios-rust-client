




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountInfoPostRequest {
    /// Array of Cardano stake address(es) in bech32 format
    #[serde(rename = "_stake_addresses")]
    pub _stake_addresses: Vec<String>,
}

impl AccountInfoPostRequest {
    pub fn new(_stake_addresses: Vec<String>) -> AccountInfoPostRequest {
        AccountInfoPostRequest {
            _stake_addresses,
        }
    }
}


