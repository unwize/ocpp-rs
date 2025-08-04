mod apn_authentication_enum_type;
mod attribute_enum_type;
mod authorization_status_enum_type;
mod authorize_certificate_status_enum_type;
mod battery_swap_event_enum_typs;
mod boot_reason_enum_type;
mod cancel_reservation_status_enum_type;
mod certificate_action_enum_type;
mod certificate_signed_status_enum_type;
mod certificate_signing_use_enum_type;
mod certificate_status_enum_type;
mod certificate_status_source_enum_type;
mod change_availability_status_enum_type;
mod charging_profile_kind_enum_type;
mod charging_profile_purpose_enum_type;
mod charging_profile_status_enum_type;
mod charging_rate_unit_enum_type;

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