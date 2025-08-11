use serde::{Deserialize, Serialize};
use crate::enums::location_enum_type::LocationEnumType;
use crate::enums::phase_enum_type::PhaseEnumType;
use crate::enums::reading_context_enum_type::ReadingContextEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::signed_meter_value_type::SignedMeterValueType;
use crate::traits::OcppEntity;

/// Single sampled value in MeterValues. Each value can be accompanied by optional fields.
/// Used by: Common:MeterValueType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    /// Required. Indicates the measured value.
    pub value: f64,
    /// Optional. Type of measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,
    /// Optional. Type of detail value: start, end or sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,
    /// Optional. Indicates how the measured value is to be interpreted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,
    /// Optional. Indicates where the measured value has been sampled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,
    /// Optional. Contains the MeterValueSignature with sign/encoding method information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
    /// Optional. Represents a UnitOfMeasure including a multiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasureType>,
}

impl OcppEntity for SampledValueType {
    /// Validates the fields of SampledValueType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(signed_value) = &self.signed_meter_value {
            e.check_member("signed_meter_value", signed_value);
        }
        if let Some(unit) = &self.unit_of_measure {
            e.check_member("unit_of_measure", unit);
        }

        e.build("SampledValueType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;


    #[test]
    fn test_validate_success_full() {
        let sampled_value = Default::default();
        assert!(sampled_value.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let sampled_value = SampledValueType {
            value: 100.0,
            measurand: None,
            context: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: None,
        };
        assert!(sampled_value.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = Default::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SampledValueType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}