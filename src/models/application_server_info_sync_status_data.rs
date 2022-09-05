/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationServerInfoSyncStatusData : Detailed sync status



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationServerInfoSyncStatusData {
    /// The CryptoCode of the crypto currency (eg. BTC)
    #[serde(rename = "cryptoCode", skip_serializing_if = "Option::is_none")]
    pub crypto_code: Option<String>,
    #[serde(rename = "nodeInformation", skip_serializing_if = "Option::is_none")]
    pub node_information: Option<Box<crate::models::ApplicationServerInfoNodeStatusData>>,
    /// The height of the chain of header of the internal indexer
    #[serde(rename = "chainHeight", skip_serializing_if = "Option::is_none")]
    pub chain_height: Option<i32>,
    /// The height of the latest indexed block of the internal indexer
    #[serde(rename = "syncHeight", skip_serializing_if = "Option::is_none")]
    pub sync_height: Option<f32>,
    /// True if the full node and the indexer are fully synchronized
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

impl ApplicationServerInfoSyncStatusData {
    /// Detailed sync status
    pub fn new() -> ApplicationServerInfoSyncStatusData {
        ApplicationServerInfoSyncStatusData {
            crypto_code: None,
            node_information: None,
            chain_height: None,
            sync_height: None,
            available: None,
        }
    }
}


