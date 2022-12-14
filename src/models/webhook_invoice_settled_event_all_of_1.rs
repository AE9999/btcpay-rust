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
pub struct WebhookInvoiceSettledEventAllOf1 {
    /// Whether this invoice has received more money than expected
    #[serde(rename = "overPaid", skip_serializing_if = "Option::is_none")]
    pub over_paid: Option<bool>,
}

impl WebhookInvoiceSettledEventAllOf1 {
    pub fn new() -> WebhookInvoiceSettledEventAllOf1 {
        WebhookInvoiceSettledEventAllOf1 {
            over_paid: None,
        }
    }
}


