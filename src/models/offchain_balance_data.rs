/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OffchainBalanceData {
    /// The amount of current channel openings in millisatoshi
    #[serde(rename = "opening", skip_serializing_if = "Option::is_none")]
    pub opening: Option<String>,
    /// The amount that is available on the local end of active channels in millisatoshi
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
    /// The amount that is available on the remote end of active channels in millisatoshi
    #[serde(rename = "remote", skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    /// The amount of current channel closings in millisatoshi
    #[serde(rename = "closing", skip_serializing_if = "Option::is_none")]
    pub closing: Option<String>,
}

impl OffchainBalanceData {
    pub fn new() -> OffchainBalanceData {
        OffchainBalanceData {
            opening: None,
            local: None,
            remote: None,
            closing: None,
        }
    }
}

