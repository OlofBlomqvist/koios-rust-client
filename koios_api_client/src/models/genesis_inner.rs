




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenesisInner {
    /// Unique network identifier for chain
    #[serde(rename = "networkmagic", skip_serializing_if = "Option::is_none")]
    pub networkmagic: Option<String>,
    /// Network ID used at various CLI identification to distinguish between Mainnet and other networks
    #[serde(rename = "networkid", skip_serializing_if = "Option::is_none")]
    pub networkid: Option<String>,
    /// Number of slots in an epoch
    #[serde(rename = "epochlength", skip_serializing_if = "Option::is_none")]
    pub epochlength: Option<String>,
    /// Duration of a single slot (in seconds)
    #[serde(rename = "slotlength", skip_serializing_if = "Option::is_none")]
    pub slotlength: Option<String>,
    /// Maximum smallest units (lovelaces) supply for the blockchain
    #[serde(rename = "maxlovelacesupply", skip_serializing_if = "Option::is_none")]
    pub maxlovelacesupply: Option<String>,
    /// UNIX timestamp of the first block (genesis) on chain
    #[serde(rename = "systemstart", skip_serializing_if = "Option::is_none")]
    pub systemstart: Option<i32>,
    /// Active Slot Co-Efficient (f) - determines the _probability_ of number of slots in epoch that are expected to have blocks (so mainnet, this would be: 432000 * 0.05 = 21600 estimated blocks)
    #[serde(rename = "activeslotcoeff", skip_serializing_if = "Option::is_none")]
    pub activeslotcoeff: Option<String>,
    /// Number of slots that represent a single KES period (a unit used for validation of KES key evolutions)
    #[serde(rename = "slotsperkesperiod", skip_serializing_if = "Option::is_none")]
    pub slotsperkesperiod: Option<String>,
    /// Number of KES key evolutions that will automatically occur before a KES (hot) key is expired. This parameter is for security of a pool, in case an operator had access to his hot(online) machine compromised
    #[serde(rename = "maxkesrevolutions", skip_serializing_if = "Option::is_none")]
    pub maxkesrevolutions: Option<String>,
    /// A unit (k) used to divide epochs to determine stability window (used in security checks like ensuring atleast 1 block was created in 3*k/f period, or to finalize next epoch's nonce at 4*k/f slots before end of epoch)
    #[serde(rename = "securityparam", skip_serializing_if = "Option::is_none")]
    pub securityparam: Option<String>,
    /// Number of BFT members that need to approve (via vote) a Protocol Update Proposal
    #[serde(rename = "updatequorum", skip_serializing_if = "Option::is_none")]
    pub updatequorum: Option<String>,
    /// A JSON dump of Alonzo Genesis
    #[serde(rename = "alonzogenesis", skip_serializing_if = "Option::is_none")]
    pub alonzogenesis: Option<String>,
}

impl GenesisInner {
    pub fn new() -> GenesisInner {
        GenesisInner {
            networkmagic: None,
            networkid: None,
            epochlength: None,
            slotlength: None,
            maxlovelacesupply: None,
            systemstart: None,
            activeslotcoeff: None,
            slotsperkesperiod: None,
            maxkesrevolutions: None,
            securityparam: None,
            updatequorum: None,
            alonzogenesis: None,
        }
    }
}


