use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::enter_service_type::EnterServiceType;
use serde::{Deserialize, Serialize};

/// EnterServiceGetType is used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterServiceGetType {
    /// Required. Id of setting
    /// String length: 0..36
    pub id: String,
    /// Required. Enter Service settings
    pub enter_service: EnterServiceType,
}

impl EnterServiceGetType {
    /// Validates the fields of EnterServiceGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_cardinality("id", 0, 36, &self.id.chars());
        e.check_member("enter_service", &self.enter_service);
        e.build("EnterServiceGetType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let enter_service_get = EnterServiceGetType {
            id: "SERVICE_SETTING_001".to_string(),
            enter_service: EnterServiceType::default(), // Placeholder
        };

        let serialized = serde_json::to_string(&enter_service_get).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EnterServiceGetType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(enter_service_get, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let enter_service_get = EnterServiceGetType {
            id: "valid_id".to_string(),
            enter_service: EnterServiceType::default(),
        };
        assert!(enter_service_get.validate().is_ok());

        let enter_service_get_max_id_len = EnterServiceGetType {
            id: "a".repeat(36), // Valid length
            enter_service: EnterServiceType::default(),
        };
        assert!(enter_service_get_max_id_len.validate().is_ok());
    }

    #[test]
    fn test_validation_id_too_long() {
        let enter_service_get = EnterServiceGetType {
            id: "a".repeat(37), // Invalid
            enter_service: EnterServiceType::default(),
        };
        let err = enter_service_get.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["id"]);
    }
}
