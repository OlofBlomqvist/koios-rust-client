




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScriptRedeemersInnerRedeemersInner {
    /// Hash of Transaction containing the redeemer
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// The index of the redeemer pointer in the transaction
    #[serde(rename = "tx_index", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<i32>,
    /// The budget in Memory to run a script
    #[serde(rename = "unit_mem", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unit_mem: Option<Option<::std::collections::HashMap<String, crate::models::ScriptRedeemersInnerRedeemersInnerUnitMemValue>>>,
    /// The budget in Cpu steps to run a script
    #[serde(rename = "unit_steps", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unit_steps: Option<Option<::std::collections::HashMap<String, crate::models::ScriptRedeemersInnerRedeemersInnerUnitMemValue>>>,
    /// The budget in fees to run a script - the fees depend on the ExUnits and the current prices
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// What kind of validation this redeemer is used for
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    /// The Hash of the Plutus Data
    #[serde(rename = "datum_hash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub datum_hash: Option<Option<String>>,
    /// The actual data in json format
    #[serde(rename = "datum_value", skip_serializing_if = "Option::is_none")]
    pub datum_value: Option<serde_json::Value>,
}

impl ScriptRedeemersInnerRedeemersInner {
    pub fn new() -> ScriptRedeemersInnerRedeemersInner {
        ScriptRedeemersInnerRedeemersInner {
            tx_hash: None,
            tx_index: None,
            unit_mem: None,
            unit_steps: None,
            fee: None,
            purpose: None,
            datum_hash: None,
            datum_value: None,
        }
    }
}

/// What kind of validation this redeemer is used for
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Purpose {
    #[serde(rename = "spend")]
    Spend,
    #[serde(rename = "mint")]
    Mint,
    #[serde(rename = "cert")]
    Cert,
    #[serde(rename = "reward")]
    Reward,
}

impl Default for Purpose {
    fn default() -> Purpose {
        Self::Spend
    }
}

