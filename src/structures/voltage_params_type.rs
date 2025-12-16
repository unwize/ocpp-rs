use crate::enums::power_during_cessation_enum_type::PowerDuringCessationEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct VoltageParamsType {
    /// Optional. Voltage threshold for 10-min time window mean value monitoring. Mandatory if hv10MinMeanTripDelay is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hv10_min_mean_value: Option<f64>,
    /// Optional. Time in seconds the voltage is allowed to stay above the 10-min mean value. Mandatory if OverVoltageMeanValue10min is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hv10_min_mean_trip_delay: Option<f64>,
    /// Optional. Power to be fed-in during fault-ride through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_during_cessation: Option<PowerDuringCessationEnumType>,
}

#[typetag::serde]
impl OcppEntity for VoltageParamsType {
    /// Validates the fields of VoltageParamsType.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if self.hv10_min_mean_trip_delay.is_some() && self.hv10_min_mean_value.is_none() {
            e.push_relation_error(
                "hv10_min_mean_trip_delay",
                "hv10_min_mean_value",
                "`hv10_min_mean_value` must be defined if `hv10_min_mean_trip_delay` is defined!",
            );
        }

        e.build("VoltageParamsType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::power_during_cessation_enum_type::PowerDuringCessationEnumType;
    use serde_json;

    fn create_test_instance() -> VoltageParamsType {
        VoltageParamsType {
            hv10_min_mean_value: Some(253.0),
            hv10_min_mean_trip_delay: Some(50.0),
            power_during_cessation: Some(PowerDuringCessationEnumType::Active),
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_fields_present() {
        let data = VoltageParamsType {
            hv10_min_mean_value: None,
            hv10_min_mean_trip_delay: None,
            power_during_cessation: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VoltageParamsType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
