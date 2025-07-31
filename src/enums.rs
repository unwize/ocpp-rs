use crate::errors::OcppError;
use crate::errors::OcppError::InvalidEnumValueError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum APNAuthentication {
    PAP = 0, // Use PAP authentication
    CHAP = 1, // Use CHAP authentication
    NONE = 2, // Use no authentication
    AUTO = 3, // Sequentially try CHAP, PAP, NONE.
}

impl Into<String> for APNAuthentication {
    fn into(self) -> String {
        match self {
            APNAuthentication::PAP => "pap".to_string(),
            APNAuthentication::CHAP => "chap".to_string(),
            APNAuthentication::NONE => "none".to_string(),
            APNAuthentication::AUTO => "auto".to_string(),
        }
    }
}

impl TryFrom<String> for APNAuthentication {
    type Error = OcppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "pap" => Ok(APNAuthentication::PAP),
            "chap" => Ok(APNAuthentication::CHAP),
            "none" => Ok(APNAuthentication::NONE),
            "auto" => Ok(APNAuthentication::AUTO),
            _ => {Err(InvalidEnumValueError{enum_name: String::from("APNAuthentication"), value })}
        }
    }
}