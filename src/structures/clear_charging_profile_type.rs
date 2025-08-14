use crate::enums::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

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
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    /// Optional. Specifies the stackLevel for which charging profiles will be cleared,
    /// if they meet the other criteria in the request.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

impl OcppEntity for ClearChargingProfileType {
    /// Validates the fields of ClearChargingProfileType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(evse_id) = self.evse_id {
            e.check_bounds("evse_id", 0, i32::MAX, evse_id);
        }

        if let Some(stack_level) = self.stack_level {
            e.check_bounds("stack_level", 0, i32::MAX, stack_level);
        }

        e.build("ClearChargingProfileType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let clear_profile = ClearChargingProfileType {
            evse_id: Some(1),
            charging_profile_purpose: Some(ChargingProfilePurposeEnumType::TxProfile),
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
            charging_profile_purpose: Some(
                ChargingProfilePurposeEnumType::ChargingStationMaxProfile,
            ),
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
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["evse_id"]);
    }

    #[test]
    fn test_validation_invalid_stack_level() {
        let clear_profile = ClearChargingProfileType {
            evse_id: None,
            charging_profile_purpose: None,
            stack_level: Some(-1), // Invalid
        };
        let err = clear_profile.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["stack_level"]);
    }

    #[test]
    fn test_validation_multiple_errors() {
        let clear_profile = ClearChargingProfileType {
            evse_id: Some(-5), // Invalid 1
            charging_profile_purpose: Some(ChargingProfilePurposeEnumType::TxProfile),
            stack_level: Some(-2), // Invalid 2
        };
        let err = clear_profile.validate().unwrap_err();
        assert_invalid_fields(&err, &["evse_id", "stack_level"]);
    }
}
