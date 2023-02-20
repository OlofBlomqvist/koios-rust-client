




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerPlutusContractsInner {
    /// Plutus script address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "script_hash", skip_serializing_if = "Option::is_none")]
    pub script_hash: Option<String>,
    /// CBOR-encoded Plutus script data
    #[serde(rename = "bytecode", skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,
    /// The size of the CBOR serialised script (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// True if the contract is valid or there is no contract
    #[serde(rename = "valid_contract", skip_serializing_if = "Option::is_none")]
    pub valid_contract: Option<bool>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerInput>>,
    #[serde(rename = "output", default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerOutput>>,
}

impl TxInfoInnerPlutusContractsInner {
    pub fn new() -> TxInfoInnerPlutusContractsInner {
        TxInfoInnerPlutusContractsInner {
            address: None,
            script_hash: None,
            bytecode: None,
            size: None,
            valid_contract: None,
            input: None,
            output: None,
        }
    }
}


