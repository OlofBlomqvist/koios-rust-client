




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerPlutusContractsInnerInputRedeemer {
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerInputRedeemerUnit>>,
    #[serde(rename = "datum", skip_serializing_if = "Option::is_none")]
    pub datum: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerInputRedeemerDatum>>,
}

impl TxInfoInnerPlutusContractsInnerInputRedeemer {
    pub fn new() -> TxInfoInnerPlutusContractsInnerInputRedeemer {
        TxInfoInnerPlutusContractsInnerInputRedeemer {
            purpose: None,
            fee: None,
            unit: None,
            datum: None,
        }
    }
}


