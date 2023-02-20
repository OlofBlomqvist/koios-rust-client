




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerPlutusContractsInnerInputRedeemerUnit {
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[serde(rename = "mem", skip_serializing_if = "Option::is_none")]
    pub mem: Option<String>,
}

impl TxInfoInnerPlutusContractsInnerInputRedeemerUnit {
    pub fn new() -> TxInfoInnerPlutusContractsInnerInputRedeemerUnit {
        TxInfoInnerPlutusContractsInnerInputRedeemerUnit {
            steps: None,
            mem: None,
        }
    }
}


