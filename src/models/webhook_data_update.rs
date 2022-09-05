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
pub struct WebhookDataUpdate {
    /// The id of the webhook
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this webhook is enabled or not
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// If true, BTCPay Server will retry to redeliver any failed delivery after 10 seconds, 1 minutes and up to 6 times after 10 minutes.
    #[serde(rename = "automaticRedelivery", skip_serializing_if = "Option::is_none")]
    pub automatic_redelivery: Option<bool>,
    /// The endpoint where BTCPay Server will make the POST request with the webhook body
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "authorizedEvents", skip_serializing_if = "Option::is_none")]
    pub authorized_events: Option<Box<crate::models::WebhookDataBaseAuthorizedEvents>>,
    /// Must be used by the callback receiver to ensure the delivery comes from BTCPay Server. BTCPay Server includes the `BTCPay-Sig` HTTP header, whose format is `sha256=HMAC256(UTF8(webhook's secret), body)`. The pattern to authenticate the webhook is similar to [how to secure webhooks in Github](https://docs.github.com/webhooks/securing/). If left out, null, or empty, the secret will not be changed.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl WebhookDataUpdate {
    pub fn new() -> WebhookDataUpdate {
        WebhookDataUpdate {
            id: None,
            enabled: None,
            automatic_redelivery: None,
            url: None,
            authorized_events: None,
            secret: None,
        }
    }
}


