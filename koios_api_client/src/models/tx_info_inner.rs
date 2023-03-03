

use super::tx_utxos_inner::Utxo;




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInfoInner {
    /// Hash identifier of the transaction
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "block_hash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i128>,
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "epoch_slot", skip_serializing_if = "Option::is_none")]
    pub epoch_slot: Option<i128>,
    #[serde(rename = "absolute_slot", skip_serializing_if = "Option::is_none")]
    pub absolute_slot: Option<i128>,
    /// UNIX timestamp of the transaction
    #[serde(rename = "tx_timestamp", skip_serializing_if = "Option::is_none")]
    pub tx_timestamp: Option<i32>,
    /// Index of transaction within block
    #[serde(rename = "tx_block_index", skip_serializing_if = "Option::is_none")]
    pub tx_block_index: Option<i32>,
    /// Size in bytes of transaction
    #[serde(rename = "tx_size", skip_serializing_if = "Option::is_none")]
    pub tx_size: Option<i32>,
    /// Total sum of all transaction outputs (in lovelaces)
    #[serde(rename = "total_output", skip_serializing_if = "Option::is_none")]
    pub total_output: Option<String>,
    /// Total Transaction fee (in lovelaces)
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Total Deposits included in transaction (for example, if it is registering a pool/key)
    #[serde(rename = "deposit", skip_serializing_if = "Option::is_none")]
    pub deposit: Option<String>,
    /// Slot before which transaction cannot be validated (if supplied, else null)
    #[serde(rename = "invalid_before", default, skip_serializing_if = "Option::is_none")]
    pub invalid_before: Option<i32>,
    /// Slot after which transaction cannot be validated
    #[serde(rename = "invalid_after", default, skip_serializing_if = "Option::is_none")]
    pub invalid_after: Option<i32>,
    #[serde(rename = "collateral_inputs", skip_serializing_if = "Option::is_none")]
    pub collateral_inputs: Option<Vec<Utxo>>,
    #[serde(rename = "collateral_output", skip_serializing_if = "Option::is_none")]
    pub collateral_output: Option<Utxo>,
    #[serde(rename = "reference_inputs", skip_serializing_if = "Option::is_none")]
    pub reference_inputs: Option<Vec<Utxo>>,
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Utxo>>,
    /// An array of UTxO outputs created by the transaction
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::TxInfoInnerOutputsInner>>,
    /// Array of withdrawals with-in a transaction
    #[serde(rename = "withdrawals", default, skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<Vec<crate::models::TxInfoInnerWithdrawalsInner>>,
    /// Array of minted assets with-in a transaction
    #[serde(rename = "assets_minted", default, skip_serializing_if = "Option::is_none")]
    pub assets_minted: Option<Vec<crate::models::TxInfoInnerAssetsMintedInner>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::value::Value>,
    /// Certificates present with-in a transaction (if any)
    #[serde(rename = "certificates", default, skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<crate::models::TxInfoInnerCertificatesInner>>,
    /// Native scripts present in a transaction (if any)
    #[serde(rename = "native_scripts", default, skip_serializing_if = "Option::is_none")]
    pub native_scripts: Option<Vec<crate::models::TxInfoInnerNativeScriptsInner>>,
    /// Plutus contracts present in transaction (if any)
    #[serde(rename = "plutus_contracts", skip_serializing_if = "Option::is_none")]
    pub plutus_contracts: Option<Vec<crate::models::TxInfoInnerPlutusContractsInner>>,
}

impl TxInfoInner {
    pub fn new() -> TxInfoInner {
        TxInfoInner {
            tx_hash: None,
            block_hash: None,
            block_height: None,
            epoch_no: None,
            epoch_slot: None,
            absolute_slot: None,
            tx_timestamp: None,
            tx_block_index: None,
            tx_size: None,
            total_output: None,
            fee: None,
            deposit: None,
            invalid_before: None,
            invalid_after: None,
            collateral_inputs: None,
            collateral_output: None,
            reference_inputs: None,
            inputs: None,
            outputs: None,
            withdrawals: None,
            assets_minted: None,
            metadata: None,
            certificates: None,
            native_scripts: None,
            plutus_contracts: None,
        }
    }
}


