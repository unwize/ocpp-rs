use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::limit_max_discharge_type::LimitMaxDischargeType;
use crate::traits::OcppEntity;

/// Used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeGetType {
    /// Required. Id of setting
    pub id: String,
    /// Required. True if setting is a default control.
    pub is_default: bool,
    /// Required. True if this setting is superseded by a higher priority setting.
    pub is_superseded: bool,
    /// Required. Maximum discharge power as percentage or rated capability
    pub limit_max_discharge: LimitMaxDischargeType, // TODO: Implement LimitMaxDischargeType
}

impl OcppEntity for LimitMaxDischargeGetType {
    /// Validates the fields of LimitMaxDischargeGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id", 0, 36, &self.id.chars());

        e.check_member("limit_max_discharge", &self.limit_max_discharge);

        e.build("LimitMaxDischargeGetType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let limit_max_discharge_get_type = LimitMaxDischargeGetType {
            id: "setting1".to_string(),
            is_default: true,
            is_superseded: false,
            limit_max_discharge: LimitMaxDischargeType::default(),
        };
        assert!(limit_max_discharge_get_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_length() {
        let limit_max_discharge_get_type = LimitMaxDischargeGetType {
            id: "a".repeat(37), // Invalid length
            is_default: true,
            is_superseded: false,
            limit_max_discharge: LimitMaxDischargeType::default(),
        };
        let result = limit_max_discharge_get_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "id");
            } else {
                panic!("Expected FieldValidationError for 'id'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = LimitMaxDischargeGetType {
            id: "setting1".to_string(),
            is_default: true,
            is_superseded: false,
            limit_max_discharge: LimitMaxDischargeType::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: LimitMaxDischargeGetType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
