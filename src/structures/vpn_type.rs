use crate::enums::vpn_enum_type::VPNEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    /// Required. VPN Server Address. Max length of 2000.
    pub server: String,
    /// Required. VPN User. Max length of 50.
    pub user: String,
    /// Optional. VPN group. Max length of 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Required. VPN Password. Max length of 64.
    pub password: String,
    /// Required. VPN shared secret. Max length of 255.
    pub key: String,
    /// Required. Type of VPN.
    #[serde(rename = "type")]
    pub vpn_type: VPNEnumType,
}

impl Default for VPNType {
    fn default() -> VPNType {
        Self {
            server: "".to_string(),
            user: "".to_string(),
            group: None,
            password: "".to_string(),
            key: "".to_string(),
            vpn_type: VPNEnumType::Ikev2,
        }
    }
}

impl OcppEntity for VPNType {
    /// Validates the fields of VPNType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("server", 0, 2000, &self.server.chars());
        e.check_cardinality("user", 0, 50, &self.user.chars());
        e.check_cardinality("password", 0, 64, &self.password.chars());
        e.check_cardinality("key", 0, 255, &self.key.chars());

        if let Some(group) = &self.group {
            e.check_cardinality("group", 0, 50, &group.chars());
        }

        e.build("VPNType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> VPNType {
        VPNType {
            server: "vpn.example.com".to_string(),
            user: "vpnuser".to_string(),
            group: Some("defaultGroup".to_string()),
            password: "securepassword123".to_string(),
            key: "sharedsecretkey".to_string(),
            vpn_type: VPNEnumType::L2Tp,
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_optional_fields() {
        let data = VPNType {
            server: "vpn.example.com".to_string(),
            user: "vpnuser".to_string(),
            group: None,
            password: "securepassword123".to_string(),
            key: "sharedsecretkey".to_string(),
            vpn_type: VPNEnumType::L2Tp,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_server_too_long() {
        let mut data = create_test_instance();
        data.server = "a".repeat(2001);
        assert!(data.validate().is_err());
    }
    
    #[test]
    fn test_validate_failure_password_too_long() {
        let mut data = create_test_instance();
        data.password = "a".repeat(65);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VPNType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
