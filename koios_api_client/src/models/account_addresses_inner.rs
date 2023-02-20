




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountAddressesInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl AccountAddressesInner {
    pub fn new() -> AccountAddressesInner {
        AccountAddressesInner {
            stake_address: None,
            addresses: None,
        }
    }
}


