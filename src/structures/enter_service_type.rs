use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// EnterServiceType is used by: Common::EnterServiceGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnterServiceType {
    /// Required. Priority of setting (0=highest).
    /// Constraints: 0 <= val
    pub priority: i32,
    /// Required. Enter service voltage high.
    pub high_voltage: f64, // decimal
    /// Required. Enter service voltage low.
    pub low_voltage: f64, // decimal
    /// Required. Enter service frequency high.
    pub high_freq: f64, // decimal
    /// Required. Enter service frequency low.
    pub low_freq: f64, // decimal
    /// Optional. Enter service delay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>, // decimal
    /// Optional. Enter service randomized delay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_delay: Option<f64>, // decimal
    /// Optional. Enter service ramp rate in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_rate: Option<f64>, // decimal
}

impl Default for EnterServiceType {
    fn default() -> Self {
        Self {
            priority: 0,
            high_voltage: 0.0,
            low_voltage: 0.0,
            high_freq: 0.0,
            low_freq: 0.0,
            delay: None,
            random_delay: None,
            ramp_rate: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for EnterServiceType {
    /// Validates the fields of EnterServiceType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_bounds("priority", 0, i32::MAX, self.priority);

        // high_voltage should be greater than or equal to low_voltage
        if self.high_voltage < self.low_voltage {
            e.push_relation_error(
                "high_voltage",
                "low_voltage",
                "low voltage must be less than high voltage!",
            );
        }

        e.check_bounds("high_freq", 0.0, f64::MAX, self.high_freq);
        e.check_bounds("low_freq", 0.0, f64::MAX, self.low_freq);

        // high_freq should be greater than or equal to low_freq
        if self.high_freq < self.low_freq {
            e.push_relation_error(
                "high_freq",
                "low_freq",
                "high_freq must be greater than low_freq!",
            );
        }

        // Validate optional delay, random_delay, ramp_rate (assuming non-negative if present)
        if let Some(delay) = self.delay {
            e.check_bounds("delay", 0.0, f64::MAX, delay);
        }
        if let Some(random_delay) = self.random_delay {
            e.check_bounds("random_delay", 0.0, f64::MAX, random_delay);
        }
        if let Some(ramp_rate) = self.ramp_rate {
            e.check_bounds("ramp_rate", 0.0, f64::MAX, ramp_rate);
        }

        e.build("EnterServiceType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 240.0,
            low_voltage: 220.0,
            high_freq: 60.0,
            low_freq: 50.0,
            delay: Some(5.0),
            random_delay: Some(1.0),
            ramp_rate: Some(0.1),
        };

        let serialized = serde_json::to_string(&enter_service).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EnterServiceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(enter_service, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 240.0,
            low_voltage: 220.0,
            high_freq: 60.0,
            low_freq: 50.0,
            delay: None,
            random_delay: None,
            ramp_rate: None,
        };
        assert!(enter_service.validate().is_ok());

        let enter_service_full = EnterServiceType {
            priority: 10,
            high_voltage: 480.0,
            low_voltage: 400.0,
            high_freq: 60.0,
            low_freq: 50.0,
            delay: Some(10.0),
            random_delay: Some(2.5),
            ramp_rate: Some(0.5),
        };
        assert!(enter_service_full.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_voltage_range() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 200.0,
            low_voltage: 220.0, // Invalid: low > high
            high_freq: 60.0,
            low_freq: 50.0,
            delay: None,
            random_delay: None,
            ramp_rate: None,
        };
        let err = enter_service.validate().unwrap_err();
        assert_num_field_errors(&err, 2);
        assert_invalid_fields(&err, &["high_voltage", "low_voltage"]);
    }

    #[test]
    fn test_validation_invalid_frequency_range() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 240.0,
            low_voltage: 220.0,
            high_freq: 50.0,
            low_freq: 60.0, // Invalid: low > high
            delay: None,
            random_delay: None,
            ramp_rate: None,
        };
        if let Err(err) = enter_service.validate() {
            assert_invalid_fields(&err, &["high_freq", "low_freq"]);
        }
    }

    #[test]
    fn test_validation_negative_delay() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 240.0,
            low_voltage: 220.0,
            high_freq: 60.0,
            low_freq: 50.0,
            delay: Some(-1.0), // Invalid
            random_delay: None,
            ramp_rate: None,
        };
        let err = enter_service.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "delay");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let enter_service = EnterServiceType {
            priority: 1,
            high_voltage: 200.0,
            low_voltage: 220.0, // Invalid 1 (voltage_range)
            high_freq: 50.0,
            low_freq: 60.0,           // Invalid 2 (frequency_range)
            delay: Some(-1.0),        // Invalid 3
            random_delay: Some(-2.0), // Invalid 4
            ramp_rate: Some(-0.5),    // Invalid 5
        };
        assert!(enter_service.validate().is_err());
        if let Err(err) = enter_service.validate() {
            assert_invalid_fields(
                &err,
                &[
                    "low_voltage",
                    "high_voltage",
                    "low_freq",
                    "delay",
                    "random_delay",
                    "ramp_rate",
                ],
            );
        }
    }
}
