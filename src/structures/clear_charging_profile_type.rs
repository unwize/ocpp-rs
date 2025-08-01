use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// A ClearChargingProfileType is a filter for charging profiles to be cleared by ClearChargingProfileRequest.
/// Used by: ClearChargingProfileRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearChargingProfileType {
    /// Optional. Specifies the id of the EVSE for which to clear charging profiles.
    /// An evseId of zero (0) specifies the charging profile for the overall Charging Station.
    /// Absence of this parameter means the clearing applies to all charging profiles that match
    /// the other criteria in the request.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// Optional. Specifies to purpose of the charging profiles that will be cleared,
    /// if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>, // TODO: Implement ChargingProfilePurposeEnumType
    /// Optional. Specifies the stackLevel for which charging profiles will be cleared,
    /// if they meet the other criteria in the request.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

impl ClearChargingProfileType {
    /// Validates the fields of ClearChargingProfileType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate evse_id
        if let Some(id) = self.evse_id {
            if id < 0 {
                errors.push(OcppError::FieldValidationError {
                    field: "evse_id".to_string(),
                    source: vec![OcppError::FieldValueError {
                        value: id.to_string(),
                        lower: "0".to_string(),
                        upper: "MAX_INT".to_string(), // No upper bound specified
                    }],
                });
            }
        }

        // Validate stack_level
        if let Some(level) = self.stack_level {
            if level < 0 {
                errors.push(OcppError::FieldValidationError {
                    field: "stack_level".to_string(),
                    source: vec![OcppError::FieldValueError {
                        value: level.to_string(),
                        lower: "0".to_string(),
                        upper: "MAX_INT".to_string(), // No upper bound specified
                    }],
                });
            }
        }

        // TODO:
        // No validation for 'charging_profile_purpose' without its type definition.

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ClearChargingProfileType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let clear_profile = ClearChargingProfileType {
            evse_id: Some(1),
            charging_profile_purpose: Some("TxProfile".to_string()), // TODO: Placeholder
            stack_level: Some(0),
        };

        let serialized = serde_json::to_string(&clear_profile).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ClearChargingProfileType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(clear_profile, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let clear_profile_minimal = ClearChargingProfileType {
            evse_id: None,
            charging_profile_purpose: None,
            stack_level: None,
        };
        assert!(clear_profile_minimal.validate().is_ok());

        let clear_profile_full = ClearChargingProfileType {
            evse_id: Some(0), // Valid
            charging_profile_purpose: Some("ChargePointMaxProfile".to_string()), // TODO: Placeholder
            stack_level: Some(10), // Valid
        };
        assert!(clear_profile_full.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_evse_id() {
        let clear_profile = ClearChargingProfileType {
            evse_id: Some(-1), // Invalid
            charging_profile_purpose: None,
            stack_level: None,
        };
        let err = clear_profile.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "evse_id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_stack_level() {
        let clear_profile = ClearChargingProfileType {
            evse_id: None,
            charging_profile_purpose: None,
            stack_level: Some(-1), // Invalid
        };
        let err = clear_profile.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "stack_level");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let clear_profile = ClearChargingProfileType {
            evse_id: Some(-5), // Invalid 1
            charging_profile_purpose: Some("TxProfile".to_string()),
            stack_level: Some(-2), // Invalid 2
        };
        let err = clear_profile.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 2); // Expecting 2 errors
            let field_names: Vec<String> = source.iter().map(|e| {
                if let OcppError::FieldValidationError { field, .. } = e {
                    field.clone()
                } else {
                    "".to_string()
                }
            }).collect();
            assert!(field_names.contains(&"evse_id".to_string()));
            assert!(field_names.contains(&"stack_level".to_string()));
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
