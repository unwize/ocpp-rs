use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V2XChargingParametersType {
    /// Optional. Minimum charge power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_power: Option<f64>,
    /// Optional. Minimum charge power on phase L2 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_power_l2: Option<f64>,
    /// Optional. Minimum charge power on phase L3 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_power_l3: Option<f64>,
    /// Optional. Maximum charge power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_power: Option<f64>,
    /// Optional. Maximum charge power on phase L2 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_power_l2: Option<f64>,
    /// Optional. Maximum charge power on phase L3 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_power_l3: Option<f64>,
    /// Optional. Minimum discharge power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_power: Option<f64>,
    /// Optional. Minimum discharge power on phase L2 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_power_l2: Option<f64>,
    /// Optional. Minimum discharge power on phase L3 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_power_l3: Option<f64>,
    /// Optional. Maximum discharge power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_power: Option<f64>,
    /// Optional. Maximum discharge power on phase L2 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_power_l2: Option<f64>,
    /// Optional. Maximum discharge power on phase L3 in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_power_l3: Option<f64>,
    /// Optional. Minimum charge current in A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_current: Option<f64>,
    /// Optional. Maximum charge current in A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_current: Option<f64>,
    /// Optional. Minimum discharge current in A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_current: Option<f64>,
    /// Optional. Maximum discharge current in A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_current: Option<f64>,
    /// Optional. Minimum voltage in V.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_voltage: Option<f64>,
    /// Optional. Maximum voltage in V.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_voltage: Option<f64>,
    /// Optional. Requested target state of charge in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_target_energy_request: Option<f64>,
    /// Optional. Energy to minimum allowed state of charge in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_energy_request: Option<f64>,
    /// Optional. Energy to maximum allowed state of charge in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_energy_request: Option<f64>,
    /// Optional. Energy (in Wh) to minimum state of charge for V2X activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_v2x_energy_request: Option<f64>,
    /// Optional. Energy (in Wh) to maximum state of charge for V2X activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_v2x_energy_request: Option<f64>,
    /// Optional. Target state of charge at departure as a percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_soc: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for V2XChargingParametersType {
    /// Validates the fields of V2XChargingParametersType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Check for non-negative values for fields that are not on the exception list.
        if let Some(val) = self.min_charge_power {
            e.check_bounds("min_charge_power", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_charge_power_l2 {
            e.check_bounds("min_charge_power_l2", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_charge_power_l3 {
            e.check_bounds("min_charge_power_l3", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_charge_power {
            e.check_bounds("max_charge_power", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_charge_power_l2 {
            e.check_bounds("max_charge_power_l2", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_charge_power_l3 {
            e.check_bounds("max_charge_power_l3", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_discharge_power {
            e.check_bounds("min_discharge_power", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_discharge_power_l2 {
            e.check_bounds("min_discharge_power_l2", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_discharge_power_l3 {
            e.check_bounds("min_discharge_power_l3", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_discharge_power {
            e.check_bounds("max_discharge_power", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_discharge_power_l2 {
            e.check_bounds("max_discharge_power_l2", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_discharge_power_l3 {
            e.check_bounds("max_discharge_power_l3", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_charge_current {
            e.check_bounds("min_charge_current", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_charge_current {
            e.check_bounds("max_charge_current", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_discharge_current {
            e.check_bounds("min_discharge_current", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_discharge_current {
            e.check_bounds("max_discharge_current", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.min_voltage {
            e.check_bounds("min_voltage", 0.0, f64::MAX, val);
        }
        if let Some(val) = self.max_voltage {
            e.check_bounds("max_voltage", 0.0, f64::MAX, val);
        }

        // Check the bounds for target_soc.
        if let Some(val) = self.target_soc {
            e.check_bounds("target_soc", 0, 100, val);
        }

        e.build("V2XChargingParametersType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> V2XChargingParametersType {
        V2XChargingParametersType {
            min_charge_power: Some(10.0),
            min_charge_power_l2: Some(5.0),
            min_charge_power_l3: Some(5.0),
            max_charge_power: Some(50.0),
            max_charge_power_l2: Some(25.0),
            max_charge_power_l3: Some(25.0),
            min_discharge_power: Some(10.0),
            min_discharge_power_l2: Some(5.0),
            min_discharge_power_l3: Some(5.0),
            max_discharge_power: Some(50.0),
            max_discharge_power_l2: Some(25.0),
            max_discharge_power_l3: Some(25.0),
            min_charge_current: Some(10.0),
            max_charge_current: Some(50.0),
            min_discharge_current: Some(10.0),
            max_discharge_current: Some(50.0),
            min_voltage: Some(200.0),
            max_voltage: Some(400.0),
            ev_target_energy_request: Some(100.0),
            ev_min_energy_request: Some(-20.0),
            ev_max_energy_request: Some(120.0),
            ev_min_v2x_energy_request: Some(-15.0),
            ev_max_v2x_energy_request: Some(15.0),
            target_soc: Some(80),
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_fields_present() {
        let data = V2XChargingParametersType {
            min_charge_power: None,
            min_charge_power_l2: None,
            min_charge_power_l3: None,
            max_charge_power: None,
            max_charge_power_l2: None,
            max_charge_power_l3: None,
            min_discharge_power: None,
            min_discharge_power_l2: None,
            min_discharge_power_l3: None,
            max_discharge_power: None,
            max_discharge_power_l2: None,
            max_discharge_power_l3: None,
            min_charge_current: None,
            max_charge_current: None,
            min_discharge_current: None,
            max_discharge_current: None,
            min_voltage: None,
            max_voltage: None,
            ev_target_energy_request: None,
            ev_min_energy_request: None,
            ev_max_energy_request: None,
            ev_min_v2x_energy_request: None,
            ev_max_v2x_energy_request: None,
            target_soc: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_min_charge_power() {
        let mut data = create_test_instance();
        data.min_charge_power = Some(-1.0);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_target_soc_too_high() {
        let mut data = create_test_instance();
        data.target_soc = Some(101);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_min_voltage() {
        let mut data = create_test_instance();
        data.min_voltage = Some(-1.0);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: V2XChargingParametersType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
