




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatumInfoInner {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Bytes>,
}

impl DatumInfoInner {
    pub fn new() -> DatumInfoInner {
        DatumInfoInner {
            hash: None,
            value: None,
            bytes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Bytes {
    pub bytes : String,
    pub value : serde_json::value::Value
}

