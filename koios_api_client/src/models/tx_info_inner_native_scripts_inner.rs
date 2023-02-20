




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerNativeScriptsInner {
    #[serde(rename = "script_hash", skip_serializing_if = "Option::is_none")]
    pub script_hash: Option<String>,
    /// JSON representation of the timelock script (null for other script types)
    #[serde(rename = "script_json", skip_serializing_if = "Option::is_none")]
    pub script_json: Option<serde_json::Value>,
}

impl TxInfoInnerNativeScriptsInner {
    pub fn new() -> TxInfoInnerNativeScriptsInner {
        TxInfoInnerNativeScriptsInner {
            script_hash: None,
            script_json: None,
        }
    }
}


