

/// TxInfoInnerOutputsInnerInlineDatum : Allows datums to be attached to UTxO (CIP-32)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerOutputsInnerInlineDatum {
    /// Datum bytes (hex)
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    /// Value (json)
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TxInfoInnerOutputsInnerInlineDatum {
    /// Allows datums to be attached to UTxO (CIP-32)
    pub fn new() -> TxInfoInnerOutputsInnerInlineDatum {
        TxInfoInnerOutputsInnerInlineDatum {
            bytes: None,
            value: None,
        }
    }
}


