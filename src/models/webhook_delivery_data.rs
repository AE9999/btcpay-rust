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
pub struct WebhookDeliveryData {
    /// The id of the delivery
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Timestamp of when the delivery got broadcasted
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f32>,
    /// HTTP code received by the remote service, if any.
    #[serde(rename = "httpCode", skip_serializing_if = "Option::is_none")]
    pub http_code: Option<f32>,
    /// User friendly error message, if any.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Whether the delivery failed or not (possible values are: `Failed`, `HttpError`, `HttpSuccess`)
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl WebhookDeliveryData {
    pub fn new() -> WebhookDeliveryData {
        WebhookDeliveryData {
            id: None,
            timestamp: None,
            http_code: None,
            error_message: None,
            status: None,
        }
    }
}


