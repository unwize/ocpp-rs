use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// EV DC charging parameters for ISO 15118-2
/// Used by: Common::ChargingNeedsType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DCChargingParametersType {
    /// Required. Maximum current (in A) supported by the electric vehicle. Includes cable capacity.
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType:EVMaximumCurrentLimit
    pub ev_max_current: f64, // decimal
    /// Required. Maximum voltage supported by the electric vehicle.
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType:EVMaximumVoltageLimit
    pub ev_max_voltage: f64, // decimal
    /// Optional. Maximum power (in W) supported by the electric vehicle. Required for DC charging.
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType:EVMaximumPowerLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<f64>, // decimal
    /// Optional. Capacity of the electric vehicle battery (in Wh).
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType:EVEnergyCapacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<f64>, // decimal
    /// Optional. Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType:EVEnergyRequest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<f64>, // decimal
    /// Optional. Energy available in the battery (in percent of the battery capacity).
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType: DC_EVStatus: EVRESSSOC
    /// Constraints: 0 <= val <= 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_of_charge: Option<i32>, // integer
    /// Optional. Percentage of SoC at which the EV considers the battery fully charged. (possible values: 0 - 100)
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType: FullSOC
    /// Constraints: 0 <= val <= 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_soc: Option<i32>, // integer
    /// Optional. Percentage of SoC at which the EV considers a fast charging process to end. (possible values: 0 - 100)
    /// Relates to: ISO 15118-2: DC_EVChargeParameterType: BulkSOC
    /// Constraints: 0 <= val <= 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_soc: Option<i32>, // integer
}

impl DCChargingParametersType {
    /// Validates the fields of DCChargingParametersType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Assuming positive value is implied for maximums in current/voltage/power
        if self.ev_max_current <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "ev_max_current".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.ev_max_current.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        if self.ev_max_voltage <= 0.0 {
            errors.push(OcppError::FieldValidationError {
                field: "ev_max_voltage".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.ev_max_voltage.to_string(),
                    lower: ">0".to_string(),
                    upper: "INF".to_string(),
                }],
            });
        }

        // Validate ev_max_power (optional, typically > 0 if present)
        if let Some(power) = self.ev_max_power {
            if power <= 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "ev_max_power".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: power.to_string(),
                        lower: ">0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }

        // Validate ev_energy_capacity (optional, typically >= 0 if present)
        if let Some(capacity) = self.ev_energy_capacity {
            if capacity < 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "ev_energy_capacity".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: capacity.to_string(),
                        lower: "0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }

        // Validate energy_amount (optional, typically >= 0 if present)
        if let Some(amount) = self.energy_amount {
            if amount < 0.0 {
                errors.push(OcppError::FieldValidationError {
                    field: "energy_amount".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: amount.to_string(),
                        lower: "0".to_string(),
                        upper: "INF".to_string(),
                    }],
                });
            }
        }

        // Validate state_of_charge
        if let Some(soc) = self.state_of_charge {
            if soc < 0 || soc > 100 {
                errors.push(OcppError::FieldValidationError {
                    field: "state_of_charge".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: soc.to_string(),
                        lower: "0".to_string(),
                        upper: "100".to_string(),
                    }],
                });
            }
        }

        // Validate full_soc
        if let Some(full_soc) = self.full_soc {
            if full_soc < 0 || full_soc > 100 {
                errors.push(OcppError::FieldValidationError {
                    field: "full_soc".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: full_soc.to_string(),
                        lower: "0".to_string(),
                        upper: "100".to_string(),
                    }],
                });
            }
        }

        // Validate bulk_soc
        if let Some(bulk_soc) = self.bulk_soc {
            if bulk_soc < 0 || bulk_soc > 100 {
                errors.push(OcppError::FieldValidationError {
                    field: "bulk_soc".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: bulk_soc.to_string(),
                        lower: "0".to_string(),
                        upper: "100".to_string(),
                    }],
                });
            }
        }

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "DCChargingParametersType".to_string(),
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
        let dc_params = DCChargingParametersType {
            ev_max_current: 200.0,
            ev_max_voltage: 500.0,
            ev_max_power: Some(150000.0),
            ev_energy_capacity: Some(75000.0),
            energy_amount: Some(60000.0),
            state_of_charge: Some(80),
            full_soc: Some(95),
            bulk_soc: Some(80),
        };

        let serialized = serde_json::to_string(&dc_params).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: DCChargingParametersType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dc_params, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let dc_params_minimal = DCChargingParametersType {
            ev_max_current: 1.0,
            ev_max_voltage: 1.0,
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            state_of_charge: None,
            full_soc: None,
            bulk_soc: None,
        };
        assert!(dc_params_minimal.validate().is_ok());

        let dc_params_full = DCChargingParametersType {
            ev_max_current: 250.0,
            ev_max_voltage: 900.0,
            ev_max_power: Some(200000.0),
            ev_energy_capacity: Some(100000.0),
            energy_amount: Some(80000.0),
            state_of_charge: Some(50),
            full_soc: Some(100),
            bulk_soc: Some(0),
        };
        assert!(dc_params_full.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_ev_max_current() {
        let dc_params = DCChargingParametersType {
            ev_max_current: 0.0, // Invalid
            ev_max_voltage: 500.0,
            ev_max_power: None, ev_energy_capacity: None, energy_amount: None,
            state_of_charge: None, full_soc: None, bulk_soc: None,
        };
        let err = dc_params.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_max_current");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_state_of_charge() {
        let dc_params_low = DCChargingParametersType {
            ev_max_current: 200.0,
            ev_max_voltage: 500.0,
            state_of_charge: Some(-1), // Invalid
            ev_max_power: None, ev_energy_capacity: None, energy_amount: None,
            full_soc: None, bulk_soc: None,
        };
        let err_low = dc_params_low.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err_low {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "state_of_charge");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }

        let dc_params_high = DCChargingParametersType {
            ev_max_current: 200.0,
            ev_max_voltage: 500.0,
            state_of_charge: Some(101), // Invalid
            ev_max_power: None, ev_energy_capacity: None, energy_amount: None,
            full_soc: None, bulk_soc: None,
        };
        let err_high = dc_params_high.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err_high {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "state_of_charge");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let dc_params = DCChargingParametersType {
            ev_max_current: 0.0, // Invalid 1
            ev_max_voltage: 0.0, // Invalid 2
            ev_max_power: Some(-10.0), // Invalid 3
            ev_energy_capacity: Some(-5.0), // Invalid 4
            energy_amount: Some(-1.0), // Invalid 5
            state_of_charge: Some(101), // Invalid 6
            full_soc: Some(-5), // Invalid 7
            bulk_soc: Some(105), // Invalid 8
        };
        let err = dc_params.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 8); // Expecting 8 errors
            let field_names: Vec<String> = source.iter().map(|e| {
                if let OcppError::FieldValidationError { field, .. } = e {
                    field.clone()
                } else {
                    "".to_string()
                }
            }).collect();
            assert!(field_names.contains(&"ev_max_current".to_string()));
            assert!(field_names.contains(&"ev_max_voltage".to_string()));
            assert!(field_names.contains(&"ev_max_power".to_string()));
            assert!(field_names.contains(&"ev_energy_capacity".to_string()));
            assert!(field_names.contains(&"energy_amount".to_string()));
            assert!(field_names.contains(&"state_of_charge".to_string()));
            assert!(field_names.contains(&"full_soc".to_string()));
            assert!(field_names.contains(&"bulk_soc".to_string()));
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
