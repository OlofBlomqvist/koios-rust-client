




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetAddressListInner {
    /// A Cardano payment/base address (bech32 encoded) for transaction's input UTxO
    #[serde(rename = "payment_address", skip_serializing_if = "Option::is_none")]
    pub payment_address: Option<String>,
    /// Asset balance on the payment address
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}

impl AssetAddressListInner {
    pub fn new() -> AssetAddressListInner {
        AssetAddressListInner {
            payment_address: None,
            quantity: None,
        }
    }
}


