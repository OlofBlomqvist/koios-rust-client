




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolInfoInnerRelaysInner {
    /// DNS name of the relay (nullable)
    #[serde(rename = "dns", default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    /// DNS service name of the relay (nullable)
    #[serde(rename = "srv", default, skip_serializing_if = "Option::is_none")]
    pub srv: Option<String>,
    /// IPv4 address of the relay (nullable)
    #[serde(rename = "ipv4", default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    /// IPv6 address of the relay (nullable)
    #[serde(rename = "ipv6", default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    /// Port number of the relay (nullable)
    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<f32>,
}

impl PoolInfoInnerRelaysInner {
    pub fn new() -> PoolInfoInnerRelaysInner {
        PoolInfoInnerRelaysInner {
            dns: None,
            srv: None,
            ipv4: None,
            ipv6: None,
            port: None,
        }
    }
}


