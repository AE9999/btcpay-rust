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
pub struct InvoiceDataAllOf {
    /// The identifier of the invoice
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The store identifier that the invoice belongs to
    #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The amount of the invoice
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The currency of the invoice
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::InvoiceType>,
    /// The link to the checkout page, where you can redirect the customer
    #[serde(rename = "checkoutLink", skip_serializing_if = "Option::is_none")]
    pub checkout_link: Option<String>,
    /// The creation time of the invoice
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Box<f32>>,
    /// The expiration time of the invoice
    #[serde(rename = "expirationTime", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<Box<f32>>,
    /// The monitoring time of the invoice
    #[serde(rename = "monitoringTime", skip_serializing_if = "Option::is_none")]
    pub monitoring_time: Option<Box<f32>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::InvoiceStatus>,
    #[serde(rename = "additionalStatus", skip_serializing_if = "Option::is_none")]
    pub additional_status: Option<crate::models::InvoiceAdditionalStatus>,
    /// The statuses the invoice can be manually marked as
    #[serde(rename = "availableStatusesForManualMarking", skip_serializing_if = "Option::is_none")]
    pub available_statuses_for_manual_marking: Option<Vec<crate::models::InvoiceStatus>>,
    /// true if the invoice is archived
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl InvoiceDataAllOf {
    pub fn new() -> InvoiceDataAllOf {
        InvoiceDataAllOf {
            id: None,
            store_id: None,
            amount: None,
            currency: None,
            _type: None,
            checkout_link: None,
            created_time: None,
            expiration_time: None,
            monitoring_time: None,
            status: None,
            additional_status: None,
            available_statuses_for_manual_marking: None,
            archived: None,
        }
    }
}


