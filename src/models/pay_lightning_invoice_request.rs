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
pub struct PayLightningInvoiceRequest {
    /// The BOLT11 of the invoice to pay
    #[serde(rename = "BOLT11", skip_serializing_if = "Option::is_none")]
    pub bolt11: Option<String>,
    /// Optional explicit payment amount in millisatoshi (if specified, it overrides the BOLT11 amount)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The fee limit expressed as a percentage of the payment amount
    #[serde(rename = "maxFeePercent", skip_serializing_if = "Option::is_none")]
    pub max_fee_percent: Option<f32>,
    /// The fee limit expressed as a fixed amount in satoshi
    #[serde(rename = "maxFeeFlat", skip_serializing_if = "Option::is_none")]
    pub max_fee_flat: Option<String>,
}

impl PayLightningInvoiceRequest {
    pub fn new() -> PayLightningInvoiceRequest {
        PayLightningInvoiceRequest {
            bolt11: None,
            amount: None,
            max_fee_percent: None,
            max_fee_flat: None,
        }
    }
}

