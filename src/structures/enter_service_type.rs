use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// EnterServiceType is used by: Common::EnterServiceGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterServiceType {
    /// Required. Priority of setting (0=highest).
    /// Constraints: 0 <= val
    pub priority: u32,
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

impl EnterServiceType {
    /// Validates the fields of EnterServiceType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate priority
        if self.priority < 0 {
            errors.push(OcppError::FieldValidationError {
                field: "priority".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.priority.to_string(),
                    lower: "0".to_string(),
                    upper: "MAX_INT".to_string(), // No upper bound specified
                }],
            });
        }

        // Validate high_voltage (assuming it should be positive)
        if self.high_voltage <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "high_voltage".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.high_voltage.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        // Validate low_voltage (assuming it should be positive)
        if self.low_voltage <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "low_voltage".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.low_voltage.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        // high_voltage should be greater than or equal to low_voltage
        if self.high_voltage < self.low_voltage {
            errors.push(OcppError::FieldValidationError {
                field: "voltage_range".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: format!("high: {}, low: {}", self.high_voltage, self.low_voltage),
                    lower: "lowVoltage <= highVoltage".to_string(),
                    upper: "".to_string(), // No upper bound for this combined check
                }],
            });
        }

        // Validate high_freq (assuming it should be positive)
        if self.high_freq <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "high_freq".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.high_freq.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        // Validate low_freq (assuming it should be positive)
        if self.low_freq <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "low_freq".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.low_freq.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        // high_freq should be greater than or equal to low_freq
        if self.high_freq < self.low_freq {
            errors.push(OcppError::FieldValidationError {
                field: "frequency_range".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: format!("high: {}, low: {}", self.high_freq, self.low_freq),
                    lower: "lowFreq <= highFreq".to_string(),
                    upper: "".to_string(), // No upper bound for this combined check
                }],
            });
        }

        // Validate optional delay, random_delay, ramp_rate (assuming non-negative if present)
        if let Some(d) = self.delay {
            if d < 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "delay".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: d.to_string(),
                        lower: "0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }
        if let Some(rd) = self.random_delay {
            if rd < 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "random_delay".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: rd.to_string(),
                        lower: "0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }
        if let Some(rr) = self.ramp_rate {
            if rr < 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "ramp_rate".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: rr.to_string(),
                        lower: "0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EnterServiceType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::assert_invalid_fields;
    use super::*;

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
            delay: None, random_delay: None, ramp_rate: None,
        };
        let err = enter_service.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "voltage_range");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_frequency_range() {
        let enter_service = EnterServiceType {
            priority: 0,
            high_voltage: 240.0,
            low_voltage: 220.0,
            high_freq: 50.0,
            low_freq: 60.0, // Invalid: low > high
            delay: None, random_delay: None, ramp_rate: None,
        };
        let err = enter_service.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "frequency_range");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
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
            random_delay: None, ramp_rate: None,
        };
        let err = enter_service.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
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
            low_freq: 60.0, // Invalid 2 (frequency_range)
            delay: Some(-1.0), // Invalid 3
            random_delay: Some(-2.0), // Invalid 4
            ramp_rate: Some(-0.5), // Invalid 5
        };
        let err = enter_service.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "priority".to_string(),
            "voltage_range".to_string(),
            "frequency_range".to_string(),
            "delay".to_string(),
            "random_delay".to_string(),
            "ramp_rate".to_string(),
        ]);
    }
}
