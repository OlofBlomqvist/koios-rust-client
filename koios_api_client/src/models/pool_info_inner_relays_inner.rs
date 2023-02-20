




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolInfoInnerRelaysInner {
    /// DNS name of the relay (nullable)
    #[serde(rename = "dns", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dns: Option<Option<String>>,
    /// DNS service name of the relay (nullable)
    #[serde(rename = "srv", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub srv: Option<Option<String>>,
    /// IPv4 address of the relay (nullable)
    #[serde(rename = "ipv4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Option<String>>,
    /// IPv6 address of the relay (nullable)
    #[serde(rename = "ipv6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Option<String>>,
    /// Port number of the relay (nullable)
    #[serde(rename = "port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<f32>>,
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


