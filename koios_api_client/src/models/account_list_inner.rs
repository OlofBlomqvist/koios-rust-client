




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountListInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl AccountListInner {
    pub fn new() -> AccountListInner {
        AccountListInner {
            id: None,
        }
    }
}


