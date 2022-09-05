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
pub struct UsersCreateUserRequest {
    /// The email of the new user
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The password of the new user
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Make this user administrator (only if you have the `unrestricted` permission of a server administrator)
    #[serde(rename = "isAdministrator", skip_serializing_if = "Option::is_none")]
    pub is_administrator: Option<bool>,
}

impl UsersCreateUserRequest {
    pub fn new() -> UsersCreateUserRequest {
        UsersCreateUserRequest {
            email: None,
            password: None,
            is_administrator: None,
        }
    }
}


