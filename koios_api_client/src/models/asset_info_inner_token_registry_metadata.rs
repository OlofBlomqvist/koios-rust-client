

/// AssetInfoInnerTokenRegistryMetadata : Asset metadata registered on the Cardano Token Registry



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetInfoInnerTokenRegistryMetadata {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A PNG image file as a byte string
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i32>,
}

impl AssetInfoInnerTokenRegistryMetadata {
    /// Asset metadata registered on the Cardano Token Registry
    pub fn new() -> AssetInfoInnerTokenRegistryMetadata {
        AssetInfoInnerTokenRegistryMetadata {
            name: None,
            description: None,
            ticker: None,
            url: None,
            logo: None,
            decimals: None,
        }
    }
}


