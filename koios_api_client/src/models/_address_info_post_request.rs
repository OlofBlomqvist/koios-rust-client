




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressInfoPostRequest {
    /// Array of Cardano payment address(es) in bech32 format
    #[serde(rename = "_addresses")]
    pub _addresses: Vec<String>,
}

impl AddressInfoPostRequest {
    pub fn new(_addresses: Vec<String>) -> AddressInfoPostRequest {
        AddressInfoPostRequest {
            _addresses,
        }
    }
}


