




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlutusScriptListInner {
    /// Hash of a script
    #[serde(rename = "script_hash", skip_serializing_if = "Option::is_none")]
    pub script_hash: Option<String>,
    /// Hash of the script creation transaction
    #[serde(rename = "creation_tx_hash", skip_serializing_if = "Option::is_none")]
    pub creation_tx_hash: Option<String>,
}

impl PlutusScriptListInner {
    pub fn new() -> PlutusScriptListInner {
        PlutusScriptListInner {
            script_hash: None,
            creation_tx_hash: None,
        }
    }
}


