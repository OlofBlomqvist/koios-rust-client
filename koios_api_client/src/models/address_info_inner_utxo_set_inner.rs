use super::tx_utxos_inner::InlineDatum;






#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressInfoInnerUtxoSetInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "tx_index", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<i128>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i128>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "datum_hash", skip_serializing_if = "Option::is_none")]
    pub datum_hash: Option<String>,
    #[serde(rename = "inline_datum", skip_serializing_if = "Option::is_none")]
    pub inline_datum: Option<InlineDatum>,
    #[serde(rename = "reference_script", skip_serializing_if = "Option::is_none")]
    pub reference_script: Option<crate::models::tx_utxos_inner::RefScript>,
    /// Array of policy IDs and asset names
    #[serde(rename = "asset_list", skip_serializing_if = "Option::is_none")]
    pub asset_list: Option<Vec<crate::models::AssetListInner>>,
}

impl AddressInfoInnerUtxoSetInner {
    pub fn new() -> AddressInfoInnerUtxoSetInner {
        AddressInfoInnerUtxoSetInner {
            tx_hash: None,
            tx_index: None,
            block_height: None,
            block_time: None,
            value: None,
            datum_hash: None,
            inline_datum: None,
            reference_script: None,
            asset_list: None,
        }
    }
}


