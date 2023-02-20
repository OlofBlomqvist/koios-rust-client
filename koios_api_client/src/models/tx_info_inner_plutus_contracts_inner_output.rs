




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerPlutusContractsInnerOutput {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::value::Value>,
}

impl TxInfoInnerPlutusContractsInnerOutput {
    pub fn new() -> TxInfoInnerPlutusContractsInnerOutput {
        TxInfoInnerPlutusContractsInnerOutput {
            hash: None,
            value: None,
        }
    }
}


