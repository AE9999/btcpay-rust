/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpeedPolicy : `\"HighSpeed\"`: 0 confirmations (1 confirmation if RBF enabled in transaction)    `\"MediumSpeed\"`: 1 confirmation    `\"LowMediumSpeed\"`: 2 confirmations    `\"LowSpeed\"`: 6 confirmations 

/// `\"HighSpeed\"`: 0 confirmations (1 confirmation if RBF enabled in transaction)    `\"MediumSpeed\"`: 1 confirmation    `\"LowMediumSpeed\"`: 2 confirmations    `\"LowSpeed\"`: 6 confirmations 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpeedPolicy {
    #[serde(rename = "HighSpeed")]
    HighSpeed,
    #[serde(rename = "MediumSpeed")]
    MediumSpeed,
    #[serde(rename = "LowSpeed")]
    LowSpeed,
    #[serde(rename = "LowMediumSpeed")]
    LowMediumSpeed,

}

impl ToString for SpeedPolicy {
    fn to_string(&self) -> String {
        match self {
            Self::HighSpeed => String::from("HighSpeed"),
            Self::MediumSpeed => String::from("MediumSpeed"),
            Self::LowSpeed => String::from("LowSpeed"),
            Self::LowMediumSpeed => String::from("LowMediumSpeed"),
        }
    }
}

impl Default for SpeedPolicy {
    fn default() -> SpeedPolicy {
        Self::HighSpeed
    }
}



