




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxMetalabelsInner {
    /// A distinct known metalabel
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl TxMetalabelsInner {
    pub fn new() -> TxMetalabelsInner {
        TxMetalabelsInner {
            key: None,
        }
    }
}


