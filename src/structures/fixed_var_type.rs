use crate::enums::der_unit_enum_type::DERUnitEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Used by: Common::FixedVarGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FixedVarType {
    /// Required. Priority of setting (0=highest)
    pub priority: i32,
    /// Required. The value specifies a target var output interpreted as a signed percentage (-100 to 100). A negative value refers to charging, whereas a positive one refers to discharging. The value type is determined by the unit field.
    pub setpoint: f64,
    /// Required. Unit of the setpoint.
    pub unit: DERUnitEnumType,
    /// Optional. Time when this setting becomes active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. Duration in seconds that this setting is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl OcppEntity for FixedVarType {
    /// Validates the fields of FixedVarType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);
        e.check_bounds("setpoint", -100.0, 100.0, self.setpoint);

        if let Some(duration) = self.duration {
            e.check_bounds("duration", 0.0, f64::MAX, duration);
        }

        e.build("FixedVarType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::der_unit_enum_type::DERUnitEnumType;
    use chrono::{TimeZone, Utc};
    use serde_json;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_validate_success() {
        let fixed_var_type = FixedVarType {
            priority: 0,
            setpoint: 50.0,
            unit: DERUnitEnumType::PctMaxVar,
            start_time: Some(Utc::now()),
            duration: Some(100.0),
        };
        assert!(fixed_var_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_priority() {
        let fixed_var_type = FixedVarType {
            priority: -1,
            setpoint: 50.0,
            unit: DERUnitEnumType::PctMaxVar,
            start_time: None,
            duration: None,
        };
        let result = fixed_var_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_invalid_fields(&err, &["priority"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_setpoint() {
        let fixed_var_type = FixedVarType {
            priority: 0,
            setpoint: 101.0,
            unit: DERUnitEnumType::PctMaxVar,
            start_time: None,
            duration: None,
        };
        let result = fixed_var_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_invalid_fields(&err, &["setpoint"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_duration() {
        let fixed_var_type = FixedVarType {
            priority: 0,
            setpoint: 50.0,
            unit: DERUnitEnumType::PctMaxVar,
            start_time: None,
            duration: Some(-10.0),
        };
        let result = fixed_var_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_invalid_fields(&err, &["duration"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_fixed_var_type = FixedVarType {
            priority: 1,
            setpoint: -25.5,
            unit: DERUnitEnumType::PctMaxVar,
            start_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            duration: Some(600.0),
        };

        // Serialize to JSON
        let serialized = serde_json::to_string(&original_fixed_var_type).unwrap();

        // Deserialize back
        let deserialized: FixedVarType = serde_json::from_str(&serialized).unwrap();

        // Check if the original and deserialized objects are equal
        assert_eq!(original_fixed_var_type, deserialized);
    }
}
