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
pub struct StoreUserDataAllOf {
    /// The id of the user
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The role of the user. Default roles are `Owner` and `Guest`
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl StoreUserDataAllOf {
    pub fn new() -> StoreUserDataAllOf {
        StoreUserDataAllOf {
            user_id: None,
            role: None,
        }
    }
}


