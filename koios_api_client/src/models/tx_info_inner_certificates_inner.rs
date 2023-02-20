




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerCertificatesInner {
    /// Certificate index
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Type of certificate (could be delegation, stake_registration, stake_deregistraion, pool_update, pool_retire, param_proposal, reserve_MIR, treasury_MIR)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A JSON array containing information from the certificate
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}

impl TxInfoInnerCertificatesInner {
    pub fn new() -> TxInfoInnerCertificatesInner {
        TxInfoInnerCertificatesInner {
            index: None,
            r#type: None,
            info: None,
        }
    }
}


