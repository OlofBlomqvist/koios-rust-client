




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxUtxosInner {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Utxo>>,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Utxo>>,
}

impl TxUtxosInner {
    pub fn new() -> TxUtxosInner {
        TxUtxosInner {
            tx_hash: None,
            inputs: None,
            outputs: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentAddr {
    pub bech32 : String,
    pub cred : String
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RefScript {
    pub hash : String,
    pub size : i128,
    #[serde(rename = "type")]
    pub typ : String,
    pub bytes : String
}
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineDatum {
    pub bytes : String,
    pub value : serde_json::value::Value
}
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Asset {
    pub policy_id : String,
    pub asset_name : Option<String>,
    pub fingerprint : String,
    pub quantity : String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Utxo {
   pub payment_addr : PaymentAddr,
   pub stake_addr : String,
   pub tx_hash : String,
   pub tx_index : i128,
   pub value : String,
   pub datum_hash : Option<String>,
   pub inline_datum : InlineDatum
}
