




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerOutputsInnerPaymentAddr {
    /// A Cardano payment/base address (bech32 encoded) where funds were sent or change to be returned
    #[serde(rename = "bech32", skip_serializing_if = "Option::is_none")]
    pub bech32: Option<String>,
    /// Payment credential
    #[serde(rename = "cred", skip_serializing_if = "Option::is_none")]
    pub cred: Option<String>,
}

impl TxInfoInnerOutputsInnerPaymentAddr {
    pub fn new() -> TxInfoInnerOutputsInnerPaymentAddr {
        TxInfoInnerOutputsInnerPaymentAddr {
            bech32: None,
            cred: None,
        }
    }
}


