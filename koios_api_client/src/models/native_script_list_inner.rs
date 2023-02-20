




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NativeScriptListInner {
    #[serde(rename = "script_hash", skip_serializing_if = "Option::is_none")]
    pub script_hash: Option<String>,
    #[serde(rename = "creation_tx_hash", skip_serializing_if = "Option::is_none")]
    pub creation_tx_hash: Option<String>,
    /// Type of the script
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl NativeScriptListInner {
    pub fn new() -> NativeScriptListInner {
        NativeScriptListInner {
            script_hash: None,
            creation_tx_hash: None,
            r#type: None,
        }
    }
}

/// Type of the script
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "timelock")]
    Timelock,
    #[serde(rename = "multisig")]
    Multisig,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Timelock
    }
}

