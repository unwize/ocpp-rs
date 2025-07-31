use serde::{Deserialize, Serialize};

/// Collection of configuration data needed to make a data-connection over a cellular network.
/// Used by: SetNetworkProfileRequest.NetworkConnectionProfileType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnType {
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
}

impl ApnType {
    /// Validates the fields of ApnType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate required field
        if self.apn.len() > 2000 {
            // println!("Validation failed: apn length exceeds 2000.");
            return false;
        }

        // Validate optional fields if they exist
        if let Some(user_name) = &self.apn_user_name {
            if user_name.len() > 50 {
                // println!("Validation failed: apn_user_name length exceeds 50.");
                return false;
            }
        }
        if let Some(password) = &self.apn_password {
            if password.len() > 64 {
                // println!("Validation failed: apn_password length exceeds 64.");
                return false;
            }
        }

        // No specific constraints for sim_pin other than its type (i32)
        true
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let apn_config = ApnType {
            apn: "internet.example.com".to_string(),
            apn_user_name: Some("user123".to_string()),
            apn_password: Some("secure_pass".to_string()),
            sim_pin: Some(1234),
        };

        let serialized = serde_json::to_string(&apn_config).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ApnType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(apn_config, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let apn_config = ApnType {
            apn: "valid.apn".to_string(),
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
        };
        assert!(apn_config.validate());

        let apn_config_full = ApnType {
            apn: "a".repeat(2000),
            apn_user_name: Some("b".repeat(50)),
            apn_password: Some("c".repeat(64)),
            sim_pin: Some(9999),
        };
        assert!(apn_config_full.validate());
    }

    #[test]
    fn test_validation_apn_too_long() {
        let apn_config = ApnType {
            apn: "a".repeat(2001), // Too long
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
        };
        assert!(!apn_config.validate());
    }

    #[test]
    fn test_validation_apn_user_name_too_long() {
        let apn_config = ApnType {
            apn: "valid.apn".to_string(),
            apn_user_name: Some("a".repeat(51)), // Too long
            apn_password: None,
            sim_pin: None,
        };
        assert!(!apn_config.validate());
    }

    #[test]
    fn test_validation_apn_password_too_long() {
        let apn_config = ApnType {
            apn: "valid.apn".to_string(),
            apn_user_name: None,
            apn_password: Some("a".repeat(65)), // Too long
            sim_pin: None,
        };
        assert!(!apn_config.validate());
    }
}
