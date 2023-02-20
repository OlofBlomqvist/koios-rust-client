




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolInfoInnerMetaJson {
    /// Pool name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Pool ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Pool homepage URL
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Pool description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PoolInfoInnerMetaJson {
    pub fn new() -> PoolInfoInnerMetaJson {
        PoolInfoInnerMetaJson {
            name: None,
            ticker: None,
            homepage: None,
            description: None,
        }
    }
}


