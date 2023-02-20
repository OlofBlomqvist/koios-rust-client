




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatumInfoPostRequest {
    /// Array of Cardano datum hashes
    #[serde(rename = "_datum_hashes", skip_serializing_if = "Option::is_none")]
    pub _datum_hashes: Option<Vec<String>>,
}

impl DatumInfoPostRequest {
    pub fn new() -> DatumInfoPostRequest {
        DatumInfoPostRequest {
            _datum_hashes: None,
        }
    }
}


