




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressAssetsInner {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Array of policy IDs and asset names
    #[serde(rename = "asset_list", skip_serializing_if = "Option::is_none")]
    pub asset_list: Option<Vec<crate::models::AssetListInner>>,
}

impl AddressAssetsInner {
    pub fn new() -> AddressAssetsInner {
        AddressAssetsInner {
            address: None,
            asset_list: None,
        }
    }
}


