




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerPlutusContractsInnerInput {
    #[serde(rename = "redeemer", skip_serializing_if = "Option::is_none")]
    pub redeemer: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerInputRedeemer>>,
    #[serde(rename = "datum", skip_serializing_if = "Option::is_none")]
    pub datum: Option<Box<crate::models::TxInfoInnerPlutusContractsInnerInputRedeemerDatum>>,
}

impl TxInfoInnerPlutusContractsInnerInput {
    pub fn new() -> TxInfoInnerPlutusContractsInnerInput {
        TxInfoInnerPlutusContractsInnerInput {
            redeemer: None,
            datum: None,
        }
    }
}


