use crate::enums::der_unit_enum_type::DERUnitEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::der_curve_points_type::DERCurvePointsType;
use crate::structures::hysteresis_type::HysteresisType;
use crate::structures::reactive_power_params_type::ReactivePowerParamsType;
use crate::structures::voltage_params_type::VoltageParamsType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// DERCurveType is used by: Common::DERCurveGetType, Common::LimitMaxDischargeType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DERCurveType {
    /// Required. Priority of curve (0=highest).
    /// Constraints: 0 <= val
    pub priority: i32,
    /// Required. Unit of the Y-axis of DER curve.
    pub y_unit: DERUnitEnumType,
    /// Optional. Open loop response time, the time to ramp up to 90% of the new target in response to the change in voltage, in seconds.
    /// A value of 0 is used to mean no limit. When not present, the device should follow its default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>, // decimal
    /// Optional. Point in time when this curve will become activated. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. Duration in seconds that this curve will be active. Only absent when default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>, // decimal
    /// Optional. Hysteresis parameters for curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis: Option<HysteresisType>,
    /// Optional. Additional parameters for voltage curves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_params: Option<VoltageParamsType>,
    /// Optional. Additional parameters for VoltVar curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactive_power_params: Option<ReactivePowerParamsType>,
    /// Required. Coordinates of the DER curve. X-axis is determined by curveType. Y-axis is determined by yUnit.
    /// Cardinality 1..10
    pub curve_data: Vec<DERCurvePointsType>,
}

impl Default for DERCurveType {
    fn default() -> DERCurveType {
        Self {
            priority: 0,
            y_unit: DERUnitEnumType::Not_Applicable,
            response_time: None,
            start_time: None,
            duration: None,
            hysteresis: None,
            voltage_params: None,
            reactive_power_params: None,
            curve_data: vec![Default::default(); 1],
        }
    }
}
#[typetag::serde]
impl OcppEntity for DERCurveType {
    /// Validates the fields of DERCurveType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);
        e.check_cardinality("curve_data", 1, 10, &self.curve_data.iter());
        e.check_iter_member("curve_data", self.curve_data.iter());

        if let Some(hysteresis) = &self.hysteresis {
            e.check_member("hysteresis", hysteresis);
        }
        if let Some(voltage_params) = &self.voltage_params {
            e.check_member("voltage_params", voltage_params);
        }
        if let Some(reactive_power_params) = &self.reactive_power_params {
            e.check_member("reactive_power_params", reactive_power_params);
        }
        e.build("DERCurveType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let der_curve = DERCurveType {
            priority: 0,
            y_unit: DERUnitEnumType::PctMaxW, // Placeholder
            response_time: Some(1.5),
            start_time: Some(Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap()),
            duration: Some(3600.0),
            hysteresis: Some(Default::default()),
            voltage_params: Some(Default::default()),
            reactive_power_params: Some(Default::default()),
            curve_data: vec![Default::default(), Default::default()],
        };

        let serialized = serde_json::to_string(&der_curve).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: DERCurveType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(der_curve, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let der_curve_minimal = DERCurveType {
            priority: 0,
            y_unit: DERUnitEnumType::PctWAvail,
            response_time: None,
            start_time: None,
            duration: None,
            hysteresis: None,
            voltage_params: None,
            reactive_power_params: None,
            curve_data: vec![Default::default()], // Valid cardinality
        };
        assert!(der_curve_minimal.validate().is_ok());

        let der_curve_full = DERCurveType {
            priority: 1,
            y_unit: DERUnitEnumType::PctVarAvail,
            response_time: Some(0.0), // No limit
            start_time: Some(Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap()),
            duration: Some(7200.0),
            hysteresis: Some(Default::default()),
            voltage_params: Some(Default::default()),
            reactive_power_params: Some(Default::default()),
            curve_data: vec![Default::default(); 10], // Max cardinality
        };
        assert!(der_curve_full.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_priority() {
        let der_curve = DERCurveType {
            priority: -1, // Invalid
            y_unit: DERUnitEnumType::Not_Applicable,
            response_time: None,
            start_time: None,
            duration: None,
            hysteresis: None,
            voltage_params: None,
            reactive_power_params: None,
            curve_data: vec![Default::default()],
        };
        let err = der_curve.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["priority"]);
    }

    #[test]
    fn test_validation_curve_data_empty() {
        let der_curve = DERCurveType {
            priority: 0,
            y_unit: DERUnitEnumType::PctVarAvail,
            response_time: None,
            start_time: None,
            duration: None,
            hysteresis: None,
            voltage_params: None,
            reactive_power_params: None,
            curve_data: vec![], // Invalid cardinality
        };
        let err = der_curve.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["curve_data"]);
    }

    #[test]
    fn test_validation_curve_data_too_many() {
        let der_curve = DERCurveType {
            priority: 0,
            y_unit: DERUnitEnumType::PctWAvail,
            response_time: None,
            start_time: None,
            duration: None,
            hysteresis: None,
            voltage_params: None,
            reactive_power_params: None,
            curve_data: vec![Default::default(); 11], // Invalid cardinality
        };
        let err = der_curve.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "curve_data");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
