use serde::{Deserialize, Serialize};
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Updates to a ChargingSchedulePeriodType for dynamic charging profiles.
/// Used by: PullDynamicScheduleUpdateResponse, UpdateDynamicScheduleRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingScheduleUpdateType {
    /// Optional. Only when not required by the ChargingRateUnit or ChargingRateUnit.Setpoint.
    /// Internal.evse.LocalFrequency, Local.GridBalancing, Local.LoadBalancing.
    /// Charging rate limit during the schedule period. In the applicable ChargingRateUnit.
    /// This SHOULD BE a non-negative value; a negative value is only supported for bidirectional
    /// compatible systems that use a negative value to specify a discharging limit.
    /// For AC this field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    
    /// Optional. Charging rate limit on phase L2 in the applicable ChargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f64>,
    
    /// Optional. Charging rate limit on phase L3 in the applicable ChargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f64>,
    
    /// Optional. Limit in ChargingRateUnit that the EV is allowed to discharge with.
    /// Note, these are negative values in order to be consistent with setpoint, which can be positive and negative.
    /// For AC this field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit: Option<f64>,
    
    /// Optional. Limit in ChargingRateUnit on phase L2 that the EV is allowed to discharge with.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit_l2: Option<f64>,
    
    /// Optional. Limit in ChargingRateUnit on phase L3 that the EV is allowed to discharge with.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit_l3: Option<f64>,
    
    /// Optional. Setpoint in ChargingRateUnit that the EV is allowed to discharge with.
    /// When a unit and/or discharge are given in the setpoint, the following setpoint values remain within these values.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint: Option<f64>,
    
    /// Optional. Setpoint in ChargingRateUnit that the EV should follow on phase L2 as close as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_l2: Option<f64>,
    
    /// Optional. Setpoint in ChargingRateUnit that the EV should follow on phase L3 as close as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_l3: Option<f64>,
    
    /// Optional. Setpoint for reactive power (or current) in ChargingRateUnit that the EV should follow.
    /// Positive values for inductive, negative for capacitive reactive power or current.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive: Option<f64>,
    
    /// Optional. Setpoint for reactive power (or current) in ChargingRateUnit that the EV should follow on phase L2 as closely as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive_l2: Option<f64>,
    
    /// Optional. Setpoint for reactive power (or current) in ChargingRateUnit that the EV should follow on phase L3 as closely as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive_l3: Option<f64>,
}

impl OcppEntity for ChargingScheduleUpdateType {
    /// Validates the fields of ChargingScheduleUpdateType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        
        // Validate discharge_limit
        if let Some(discharge_limit) = self.discharge_limit {
            e.check_bounds("discharge_limit", f64::MIN, 0.0, discharge_limit);
        }

        // Validate discharge_limit_l2
        if let Some(discharge_limit_l2) = self.discharge_limit_l2 {
            e.check_bounds("discharge_limit_l2", f64::MIN, 0.0, discharge_limit_l2);
        }

        // Validate discharge_limit_l3
        if let Some(discharge_limit_l3) = self.discharge_limit_l3 {
            e.check_bounds("discharge_limit_l3", f64::MIN, 0.0, discharge_limit_l3);
        }

        e.build("ChargingScheduleUpdateType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::assert_invalid_fields;
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let update = ChargingScheduleUpdateType {
            limit: Some(10.0),
            limit_l2: Some(5.0),
            limit_l3: Some(5.0),
            discharge_limit: Some(-2.0),
            discharge_limit_l2: Some(-1.0),
            discharge_limit_l3: Some(-1.0),
            setpoint: Some(8.0),
            setpoint_l2: Some(4.0),
            setpoint_l3: Some(4.0),
            setpoint_reactive: Some(1.0),
            setpoint_reactive_l2: Some(0.5),
            setpoint_reactive_l3: Some(0.5),
        };

        let serialized = serde_json::to_string(&update).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingScheduleUpdateType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(update, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let update_minimal = ChargingScheduleUpdateType {
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: Some(0.0), // Valid
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        };
        assert!(update_minimal.validate().is_ok());

        let update_full = ChargingScheduleUpdateType {
            limit: Some(20.0),
            limit_l2: Some(10.0),
            limit_l3: Some(10.0),
            discharge_limit: Some(-5.0),
            discharge_limit_l2: Some(-2.5),
            discharge_limit_l3: Some(-2.5),
            setpoint: Some(15.0),
            setpoint_l2: Some(7.5),
            setpoint_l3: Some(7.5),
            setpoint_reactive: Some(3.0),
            setpoint_reactive_l2: Some(1.5),
            setpoint_reactive_l3: Some(1.5),
        };
        assert!(update_full.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_discharge_limit() {
        let update = ChargingScheduleUpdateType {
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: Some(0.1), // Invalid
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        };
        let err = update.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "discharge_limit");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_discharge_limit_l2() {
        let update = ChargingScheduleUpdateType {
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: None,
            discharge_limit_l2: Some(0.1), // Invalid
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        };
        let err = update.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "discharge_limit_l2");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_discharge_limit_l3() {
        let update = ChargingScheduleUpdateType {
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: Some(0.1), // Invalid
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        };
        let err = update.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "discharge_limit_l3");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let update = ChargingScheduleUpdateType {
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: Some(1.0), // Invalid 1
            discharge_limit_l2: Some(2.0), // Invalid 2
            discharge_limit_l3: Some(3.0), // Invalid 3
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        };
        let err = update.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "discharge_limit".to_string(),
            "discharge_limit_l2".to_string(),
            "discharge_limit_l3".to_string()
        ]);
    }
}
