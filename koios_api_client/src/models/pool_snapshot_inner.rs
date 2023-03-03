




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolSnapshotInner {
    /// Type of snapshot (\"Mark\", \"Set\" or \"Go\")
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    /// Epoch number for the snapshot entry
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Pool's Active Stake for the given epoch
    #[serde(rename = "pool_stake", skip_serializing_if = "Option::is_none")]
    pub pool_stake: Option<String>,
    /// Total Active Stake for the given epoch
    #[serde(rename = "active_stake", skip_serializing_if = "Option::is_none")]
    pub active_stake: Option<String>,
}

impl PoolSnapshotInner {
    pub fn new() -> PoolSnapshotInner {
        PoolSnapshotInner {
            snapshot: None,
            epoch_no: None,
            nonce: None,
            pool_stake: None,
            active_stake: None,
        }
    }
}


