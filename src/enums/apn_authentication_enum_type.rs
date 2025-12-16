use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum APNAuthenticationEnumType {
    /// Use PAP authentication
    Pap,
    /// Use CHAP authentication
    Chap,
    /// Use no authentication
    None,
    /// Sequentially try CHAP, PAP, NONE.
    Auto,
}

impl TryFrom<String> for APNAuthenticationEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "PAP" => Ok(APNAuthenticationEnumType::Pap),
            "CHAP" => Ok(APNAuthenticationEnumType::Chap),
            "NONE" => Ok(APNAuthenticationEnumType::None),
            "AUTO" => Ok(APNAuthenticationEnumType::Auto),
            _ => Err(format!("'{}' is not a valid APNAuthenticationEnumType", s)),
        }
    }
}

impl From<APNAuthenticationEnumType> for String {
    fn from(val: APNAuthenticationEnumType) -> Self {
        match val {
            APNAuthenticationEnumType::Pap => "PAP".to_string(),
            APNAuthenticationEnumType::Chap => "CHAP".to_string(),
            APNAuthenticationEnumType::None => "NONE".to_string(),
            APNAuthenticationEnumType::Auto => "AUTO".to_string(),
        }
    }
}
