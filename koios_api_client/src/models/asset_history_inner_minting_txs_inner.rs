




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetHistoryInnerMintingTxsInner {
    /// Hash of minting/burning transaction
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i128>,
    /// Quantity minted/burned (negative numbers indicate burn transactions)
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Array of Transaction Metadata for given transaction
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<serde_json::value::Value>>,
}

impl AssetHistoryInnerMintingTxsInner {
    pub fn new() -> AssetHistoryInnerMintingTxsInner {
        AssetHistoryInnerMintingTxsInner {
            tx_hash: None,
            block_time: None,
            quantity: None,
            metadata: None,
        }
    }
}


