use crate::enums::apn_authentication_enum_type::APNAuthenticationEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Collection of configuration data needed to make a data-connection over a cellular network.
/// Used by: SetNetworkProfileRequest.NetworkConnectionProfileType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct APNType {
    /// Required. The Access Point Name as a URL.
    /// String length: 0..2000
    pub apn: String,
    /// Optional. APN username.
    /// String length: 0..50
    pub apn_user_name: Option<String>,
    /// Optional. APN Password.
    /// String length: 0..64
    pub apn_password: Option<String>,
    /// Optional. SIM card pin code.
    pub sim_pin: Option<i32>, // Assuming integer can be i32 or similar
    /// Optional. Preferred network, written as MCC and MNC concatenated.
    /// String length: 0..6
    pub preferred_network: Option<String>,
    /// Optional. Default: false. Use only the preferred Network, do not dial in when not available.
    pub use_only_preferred_network: Option<bool>,
    /// Required. Authentication method.
    pub apn_authentication: APNAuthenticationEnumType,
}

impl Default for APNType {
    fn default() -> APNType {
        Self {
            apn: "".to_string(),
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::Pap,
        }
    }
}
#[typetag::serde]
impl OcppEntity for APNType {
    /// Validates the fields of ApnType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("apn", 0, 2000, &self.apn.chars());

        if let Some(user_name) = &self.apn_user_name {
            e.check_cardinality("user_name", 0, 50, &user_name.chars());
        }
        if let Some(password) = &self.apn_password {
            e.check_cardinality("apn_password", 0, 64, &password.chars());
        }
        if let Some(preferred_net) = &self.preferred_network {
            e.check_cardinality("preferred_network", 0, 6, &preferred_net.chars());
        }

        e.build("ApnType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let apn_config = APNType {
            apn: "internet.example.com".to_string(),
            apn_user_name: Some("user123".to_string()),
            apn_password: Some("secure_pass".to_string()),
            sim_pin: Some(1234),
            preferred_network: Some("20404".to_string()),
            use_only_preferred_network: Some(true),
            apn_authentication: APNAuthenticationEnumType::try_from("CHAP".to_string()).unwrap(), // Example value
        };

        let serialized = serde_json::to_string(&apn_config).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: APNType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(apn_config, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let apn_config = APNType {
            apn: "valid.apn".to_string(),
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::try_from("NONE".to_string()).unwrap(),
        };
        assert!(apn_config.validate().is_ok());

        let apn_config_full = APNType {
            apn: "a".repeat(2000),
            apn_user_name: Some("b".repeat(50)),
            apn_password: Some("c".repeat(64)),
            sim_pin: Some(9999),
            preferred_network: Some("123456".to_string()),
            use_only_preferred_network: Some(false),
            apn_authentication: APNAuthenticationEnumType::try_from("PAP".to_string()).unwrap(),
        };
        assert!(apn_config_full.validate().is_ok());
    }

    #[test]
    fn test_validation_apn_too_long() {
        let apn_config = APNType {
            apn: "a".repeat(2001), // Too long
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::try_from("CHAP".to_string()).unwrap(),
        };
        assert!(apn_config.validate().is_err());
    }

    #[test]
    fn test_validation_apn_user_name_too_long() {
        let apn_config = APNType {
            apn: "valid.apn".to_string(),
            apn_user_name: Some("a".repeat(51)), // Too long
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::try_from("CHAP".to_string()).unwrap(),
        };
        assert!(apn_config.validate().is_err());
    }

    #[test]
    fn test_validation_apn_password_too_long() {
        let apn_config = APNType {
            apn: "valid.apn".to_string(),
            apn_user_name: None,
            apn_password: Some("a".repeat(65)), // Too long
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::try_from("CHAP".to_string()).unwrap(),
        };
        assert!(apn_config.validate().is_err());
    }

    #[test]
    fn test_validation_preferred_network_too_long() {
        let apn_config = APNType {
            apn: "valid.apn".to_string(),
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: Some("1234567".to_string()), // Too long
            use_only_preferred_network: None,
            apn_authentication: APNAuthenticationEnumType::try_from("CHAP".to_string()).unwrap(),
        };
        assert!(apn_config.validate().is_err());
    }
}
