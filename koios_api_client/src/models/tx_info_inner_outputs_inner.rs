




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInnerOutputsInner {
    #[serde(rename = "payment_addr", skip_serializing_if = "Option::is_none")]
    pub payment_addr: Option<Box<crate::models::TxInfoInnerOutputsInnerPaymentAddr>>,
    #[serde(rename = "stake_addr", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stake_addr: Option<Option<Box<String>>>,
    /// Hash of transaction for UTxO
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Index of UTxO in the transaction
    #[serde(rename = "tx_index", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<i32>,
    /// Total sum of ADA on the UTxO
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Hash of datum (if any) connected to UTxO
    #[serde(rename = "datum_hash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub datum_hash: Option<Option<String>>,
    #[serde(rename = "inline_datum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub inline_datum: Option<Option<Box<crate::models::TxInfoInnerOutputsInnerInlineDatum>>>,
    #[serde(rename = "reference_script", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reference_script: Option<Option<Box<crate::models::TxInfoInnerOutputsInnerReferenceScript>>>,
    /// An array of assets on the UTxO
    #[serde(rename = "asset_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_list: Option<Option<Vec<crate::models::TxInfoInnerOutputsInnerAssetListInner>>>,
}

impl TxInfoInnerOutputsInner {
    pub fn new() -> TxInfoInnerOutputsInner {
        TxInfoInnerOutputsInner {
            payment_addr: None,
            stake_addr: None,
            tx_hash: None,
            tx_index: None,
            value: None,
            datum_hash: None,
            inline_datum: None,
            reference_script: None,
            asset_list: None,
        }
    }
}


