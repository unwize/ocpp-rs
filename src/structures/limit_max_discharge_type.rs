use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::der_curve_type::DERCurveType;
use crate::traits::OcppEntity;

/// Used by: Common::LimitMaxDischargeGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeType {
    /// Required. Priority of setting (0=highest)
    pub priority: i32,
    /// Optional. The value specifies a percentage (0 to 100) of the rated maximum discharge power of EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pct_max_discharge_power: Option<f64>,
    /// Optional. Time when this setting becomes active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. Duration in seconds that this setting is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Optional. The curve is an interpolation of data points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_monitoring_must_trip: Option<DERCurveType>,
}

impl Default for LimitMaxDischargeType {
    fn default() -> Self {
        Self {
            priority: 0,
            pct_max_discharge_power: None,
            start_time: None,
            duration: None,
            power_monitoring_must_trip: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for LimitMaxDischargeType {
    /// Validates the fields of LimitMaxDischargeType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);

        if let Some(pct) = self.pct_max_discharge_power {
            e.check_bounds("pct_max_discharge_power", 0.0, 100.0, pct);
        }

        if let Some(curve) = &self.power_monitoring_must_trip {
            e.check_member("power_monitoring_must_trip", curve);
        }

        e.build("LimitMaxDischargeType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let limit_max_discharge_type = LimitMaxDischargeType {
            priority: 0,
            pct_max_discharge_power: Some(50.0),
            start_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            duration: Some(3600.0),
            power_monitoring_must_trip: Some(DERCurveType::default()),
        };

        if let Err(e) = limit_max_discharge_type.validate() {
            match e {
                OcppError::StructureValidationError { related, .. } => {
                    println!("{:#?}", related);
                }
                _ => {}
            }
        }

        assert!(limit_max_discharge_type.validate().is_ok())
    }

    #[test]
    fn test_validate_success_minimal() {
        let limit_max_discharge_type = LimitMaxDischargeType {
            priority: 0,
            pct_max_discharge_power: None,
            start_time: None,
            duration: None,
            power_monitoring_must_trip: None,
        };
        assert!(limit_max_discharge_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_priority() {
        let limit_max_discharge_type = LimitMaxDischargeType {
            priority: -1, // Invalid
            pct_max_discharge_power: None,
            start_time: None,
            duration: None,
            power_monitoring_must_trip: None,
        };
        let result = limit_max_discharge_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "priority");
            } else {
                panic!("Expected FieldValidationError for 'priority'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_pct_max_discharge_power_high() {
        let limit_max_discharge_type = LimitMaxDischargeType {
            priority: 0,
            pct_max_discharge_power: Some(101.0), // Invalid
            start_time: None,
            duration: None,
            power_monitoring_must_trip: None,
        };
        let result = limit_max_discharge_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "pct_max_discharge_power");
            } else {
                panic!("Expected FieldValidationError for 'pct_max_discharge_power'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = LimitMaxDischargeType {
            priority: 0,
            pct_max_discharge_power: Some(50.0),
            start_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            duration: Some(3600.0),
            power_monitoring_must_trip: Some(DERCurveType::default()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: LimitMaxDischargeType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
