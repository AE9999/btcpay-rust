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
pub struct OnChainPaymentMethodDataWithSensitiveData {
    /// Whether the payment method is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The payment method
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The derivation scheme
    #[serde(rename = "derivationScheme", skip_serializing_if = "Option::is_none")]
    pub derivation_scheme: Option<String>,
    /// A label that will be shown in the UI
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The wallet fingerprint followed by the keypath to derive the account key used for signing operation or creating PSBTs
    #[serde(rename = "accountKeyPath", skip_serializing_if = "Option::is_none")]
    pub account_key_path: Option<String>,
    /// Crypto code of the payment method
    #[serde(rename = "cryptoCode", skip_serializing_if = "Option::is_none")]
    pub crypto_code: Option<String>,
    /// The mnemonic used to generate the wallet
    #[serde(rename = "mnemonic", skip_serializing_if = "Option::is_none")]
    pub mnemonic: Option<String>,
}

impl OnChainPaymentMethodDataWithSensitiveData {
    pub fn new() -> OnChainPaymentMethodDataWithSensitiveData {
        OnChainPaymentMethodDataWithSensitiveData {
            enabled: None,
            payment_method: None,
            derivation_scheme: None,
            label: None,
            account_key_path: None,
            crypto_code: None,
            mnemonic: None,
        }
    }
}

