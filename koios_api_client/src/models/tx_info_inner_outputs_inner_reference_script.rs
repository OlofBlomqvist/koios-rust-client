

/// TxInfoInnerOutputsInnerReferenceScript : Allow reference scripts to be used to satisfy script requirements during validation, rather than requiring the spending transaction to do so. (CIP-33)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerOutputsInnerReferenceScript {
    /// Hash of referenced script
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Size in bytes
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Type of script
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Script bytes (hex)
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    /// Value (json)
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TxInfoInnerOutputsInnerReferenceScript {
    /// Allow reference scripts to be used to satisfy script requirements during validation, rather than requiring the spending transaction to do so. (CIP-33)
    pub fn new() -> TxInfoInnerOutputsInnerReferenceScript {
        TxInfoInnerOutputsInnerReferenceScript {
            hash: None,
            size: None,
            r#type: None,
            bytes: None,
            value: None,
        }
    }
}


