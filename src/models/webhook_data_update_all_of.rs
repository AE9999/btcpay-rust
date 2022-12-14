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
pub struct WebhookDataUpdateAllOf {
    /// Must be used by the callback receiver to ensure the delivery comes from BTCPay Server. BTCPay Server includes the `BTCPay-Sig` HTTP header, whose format is `sha256=HMAC256(UTF8(webhook's secret), body)`. The pattern to authenticate the webhook is similar to [how to secure webhooks in Github](https://docs.github.com/webhooks/securing/). If left out, null, or empty, the secret will not be changed.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl WebhookDataUpdateAllOf {
    pub fn new() -> WebhookDataUpdateAllOf {
        WebhookDataUpdateAllOf {
            secret: None,
        }
    }
}


