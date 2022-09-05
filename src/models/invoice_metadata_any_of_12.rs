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
pub struct InvoiceMetadataAnyOf12 {
    #[serde(rename = "itemCode", skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
}

impl InvoiceMetadataAnyOf12 {
    pub fn new() -> InvoiceMetadataAnyOf12 {
        InvoiceMetadataAnyOf12 {
            item_code: None,
        }
    }
}


