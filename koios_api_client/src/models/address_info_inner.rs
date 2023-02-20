




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressInfoInner {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Sum of all UTxO values beloning to address
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(rename = "stake_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<Option<Box<String>>>,
    /// Signifies whether the address is a script address
    #[serde(rename = "script_address", skip_serializing_if = "Option::is_none")]
    pub script_address: Option<bool>,
    #[serde(rename = "utxo_set", skip_serializing_if = "Option::is_none")]
    pub utxo_set: Option<Vec<crate::models::AddressInfoInnerUtxoSetInner>>,
}

impl AddressInfoInner {
    pub fn new() -> AddressInfoInner {
        AddressInfoInner {
            address: None,
            balance: None,
            stake_address: None,
            script_address: None,
            utxo_set: None,
        }
    }
}


