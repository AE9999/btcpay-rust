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
pub struct WebhookInvoiceReceivedPaymentEventAllOf {
    /// Whether this payment has been sent after expiration of the invoice
    #[serde(rename = "afterExpiration", skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<bool>,
    /// What payment method was used for this payment
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "payment", skip_serializing_if = "Option::is_none")]
    pub payment: Option<Box<crate::models::Payment>>,
}

impl WebhookInvoiceReceivedPaymentEventAllOf {
    pub fn new() -> WebhookInvoiceReceivedPaymentEventAllOf {
        WebhookInvoiceReceivedPaymentEventAllOf {
            after_expiration: None,
            payment_method: None,
            payment: None,
        }
    }
}

