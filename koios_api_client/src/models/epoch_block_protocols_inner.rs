




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EpochBlockProtocolsInner {
    /// Protocol major version
    #[serde(rename = "proto_major", skip_serializing_if = "Option::is_none")]
    pub proto_major: Option<i128>,
    /// Protocol major version
    #[serde(rename = "proto_minor", skip_serializing_if = "Option::is_none")]
    pub proto_minor: Option<i128>,
    /// Amount of blocks with specified major and protocol combination
    #[serde(rename = "blocks", skip_serializing_if = "Option::is_none")]
    pub blocks: Option<i128>,
}

impl EpochBlockProtocolsInner {
    pub fn new() -> EpochBlockProtocolsInner {
        EpochBlockProtocolsInner {
            proto_major: None,
            proto_minor: None,
            blocks: None,
        }
    }
}


