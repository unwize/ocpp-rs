use crate::enums::ocpp_interface_enum_type::OCPPInterfaceEnumType;
use crate::enums::ocpp_transport_enum_type::OCPPTransportEnumType;
use crate::enums::ocpp_version_enum_type::OCPPVersionEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::apn_type::APNType;
use crate::structures::vpn_type::VPNType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Defines the functional and technical parameters of a communication link.
/// Used by: SetNetworkProfileRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    /// Optional. This field is ignored, since the OCPP version to use is determined during the websocket handshake.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocpp_version: Option<OCPPVersionEnumType>,
    /// Required. Applicable Network Interface.
    pub ocpp_interface: OCPPInterfaceEnumType,
    /// Required. Defines the transport protocol.
    pub ocpp_transport: OCPPTransportEnumType,
    /// Required. Duration in seconds before a message sent by the Charging Station via this network connection times out.
    pub message_timeout: i32,
    /// Required. URL of the CSMS.
    pub ocpp_csms_url: String,
    /// Required. The security profile used.
    pub security_profile: i32,
    /// Optional. Charging Station identity to be used as the basic authentication username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Optional. BasicAuthPassword to use for security profile 1 or 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_password: Option<String>,
    /// Optional. Settings to be used to set up the VPN connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,
    /// Optional. Configuration data needed to make a data-connection over a cellular network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,
}

impl OcppEntity for NetworkConnectionProfileType {
    /// Validates the fields of NetworkConnectionProfileType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("message_timeout", 0, i32::MAX, self.message_timeout);
        e.check_cardinality("ocpp_csms_url", 0, 2000, &self.ocpp_csms_url.chars());
        e.check_bounds("security_profile", 0, i32::MAX, self.security_profile);

        if let Some(identity) = &self.identity {
            e.check_cardinality("identity", 0, 48, &identity.chars());
        }

        if let Some(password) = &self.basic_auth_password {
            e.check_cardinality("basic_auth_password", 0, 64, &password.chars());
        }

        e.build("NetworkConnectionProfileType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::apn_type::APNType;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let profile = NetworkConnectionProfileType {
            ocpp_version: Some(OCPPVersionEnumType::OCPP20),
            ocpp_interface: OCPPInterfaceEnumType::Any,
            ocpp_transport: OCPPTransportEnumType::JSON,
            message_timeout: 30,
            ocpp_csms_url: "wss://csms.example.com".to_string(),
            security_profile: 1,
            identity: Some("user".to_string()),
            basic_auth_password: Some("password".to_string()),
            vpn: Some(VPNType::default()),
            apn: Some(APNType::default()),
        };
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let profile = NetworkConnectionProfileType {
            ocpp_version: None,
            ocpp_interface: OCPPInterfaceEnumType::Any,
            ocpp_transport: OCPPTransportEnumType::JSON,
            message_timeout: 30,
            ocpp_csms_url: "wss://csms.example.com".to_string(),
            security_profile: 1,
            identity: None,
            basic_auth_password: None,
            vpn: None,
            apn: None,
        };
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_ocpp_csms_url_length() {
        let profile = NetworkConnectionProfileType {
            ocpp_version: None,
            ocpp_interface: OCPPInterfaceEnumType::Any,
            ocpp_transport: OCPPTransportEnumType::JSON,
            message_timeout: 30,
            ocpp_csms_url: "a".repeat(2001),
            security_profile: 1,
            identity: None,
            basic_auth_password: None,
            vpn: None,
            apn: None,
        };
        let result = profile.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "ocpp_csms_url");
            } else {
                panic!("Expected FieldValidationError for 'ocpp_csms_url'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_security_profile_bounds() {
        let profile = NetworkConnectionProfileType {
            ocpp_version: None,
            ocpp_interface: OCPPInterfaceEnumType::Any,
            ocpp_transport: OCPPTransportEnumType::JSON,
            message_timeout: 30,
            ocpp_csms_url: "wss://csms.example.com".to_string(),
            security_profile: -1, // Invalid
            identity: None,
            basic_auth_password: None,
            vpn: None,
            apn: None,
        };
        let result = profile.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "security_profile");
            } else {
                panic!("Expected FieldValidationError for 'security_profile'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = NetworkConnectionProfileType {
            ocpp_version: Some(OCPPVersionEnumType::OCPP20),
            ocpp_interface: OCPPInterfaceEnumType::Any,
            ocpp_transport: OCPPTransportEnumType::JSON,
            message_timeout: 30,
            ocpp_csms_url: "wss://csms.example.com".to_string(),
            security_profile: 1,
            identity: Some("user".to_string()),
            basic_auth_password: Some("password".to_string()),
            vpn: Some(VPNType::default()),
            apn: Some(APNType::default()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: NetworkConnectionProfileType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
