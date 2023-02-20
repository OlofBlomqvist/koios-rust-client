




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScriptRedeemersInner {
    /// Hash of Transaction for which details are being shown
    #[serde(rename = "script_hash", skip_serializing_if = "Option::is_none")]
    pub script_hash: Option<String>,
    #[serde(rename = "redeemers", skip_serializing_if = "Option::is_none")]
    pub redeemers: Option<Vec<crate::models::ScriptRedeemersInnerRedeemersInner>>,
}

impl ScriptRedeemersInner {
    pub fn new() -> ScriptRedeemersInner {
        ScriptRedeemersInner {
            script_hash: None,
            redeemers: None,
        }
    }
}


