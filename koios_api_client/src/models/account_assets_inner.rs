




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountAssetsInner {
    #[serde(rename = "stake_address", skip_serializing_if = "Option::is_none")]
    pub stake_address: Option<String>,
    #[serde(rename = "asset_list", skip_serializing_if = "Option::is_none")]
    pub asset_list: Option<Vec<crate::models::AccountAssetsInnerAssetListInner>>,
}

impl AccountAssetsInner {
    pub fn new() -> AccountAssetsInner {
        AccountAssetsInner {
            stake_address: None,
            asset_list: None,
        }
    }
}


