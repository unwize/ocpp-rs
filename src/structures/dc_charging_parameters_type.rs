use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

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

impl OcppEntity for DCChargingParametersType {
    /// Validates the fields of DCChargingParametersType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Assuming positive value is implied for maximums in current/voltage/power
        e.check_bounds(
            "ev_max_current",
            f64::EPSILON,
            f64::MAX,
            self.ev_max_current,
        );
        e.check_bounds(
            "ev_max_voltage",
            f64::EPSILON,
            f64::MAX,
            self.ev_max_voltage,
        );

        // Validate ev_max_power (optional, typically > 0 if present)
        if let Some(ev_max_power) = self.ev_max_power {
            e.check_bounds("ev_max_power", f64::EPSILON, f64::MAX, ev_max_power);
        }

        // Validate ev_energy_capacity (optional, typically >= 0 if present)
        if let Some(ev_energy_capacity) = self.ev_energy_capacity {
            e.check_bounds("ev_energy_capacity", 0.0, f64::MAX, ev_energy_capacity);
        }

        // Validate energy_amount (optional, typically >= 0 if present)
        if let Some(energy_amount) = self.energy_amount {
            e.check_bounds("energy_amount", 0.0, f64::MAX, energy_amount);
        }

        // Validate state_of_charge
        if let Some(state_of_charge) = self.state_of_charge {
            e.check_bounds("state_of_charge", 0, 100, state_of_charge);
        }

        // Validate full_soc
        if let Some(full_soc) = self.full_soc {
            e.check_bounds("full_soc", 0, 100, full_soc);
        }

        // Validate bulk_soc
        if let Some(bulk_soc) = self.bulk_soc {
            e.check_bounds("bulk_soc", 0, 100, bulk_soc);
        }

        e.build("DCChargingParametersType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

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
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            state_of_charge: None,
            full_soc: None,
            bulk_soc: None,
        };
        let err = dc_params.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["ev_max_current"]);
    }

    #[test]
    fn test_validation_invalid_state_of_charge() {
        let dc_params_low = DCChargingParametersType {
            ev_max_current: 200.0,
            ev_max_voltage: 500.0,
            state_of_charge: Some(-1), // Invalid
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            full_soc: None,
            bulk_soc: None,
        };
        let err_low = dc_params_low.validate().unwrap_err();
        assert_num_field_errors(&err_low, 1);
        assert_invalid_fields(&err_low, &["state_of_charge"]);

        let dc_params_high = DCChargingParametersType {
            ev_max_current: 200.0,
            ev_max_voltage: 500.0,
            state_of_charge: Some(101), // Invalid
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            full_soc: None,
            bulk_soc: None,
        };
        let err_high = dc_params_high.validate().unwrap_err();
        assert_num_field_errors(&err_high, 1);
        assert_invalid_fields(&err_high, &["state_of_charge"]);
    }

    #[test]
    fn test_validation_multiple_errors() {
        let dc_params = DCChargingParametersType {
            ev_max_current: 0.0,            // Invalid 1
            ev_max_voltage: 0.0,            // Invalid 2
            ev_max_power: Some(-10.0),      // Invalid 3
            ev_energy_capacity: Some(-5.0), // Invalid 4
            energy_amount: Some(-1.0),      // Invalid 5
            state_of_charge: Some(101),     // Invalid 6
            full_soc: Some(-5),             // Invalid 7
            bulk_soc: Some(105),            // Invalid 8
        };
        let err = dc_params.validate().unwrap_err();
        assert_invalid_fields(
            &err,
            &[
                "ev_max_current",
                "ev_max_voltage",
                "ev_max_power",
                "ev_energy_capacity",
                "energy_amount",
                "state_of_charge",
                "full_soc",
                "bulk_soc",
            ],
        );
    }
}
