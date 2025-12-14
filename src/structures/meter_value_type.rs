use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::sampled_meter_value_type::SampledValueType;
use crate::traits::OcppEntity;

/// Collection of one or more sampled values in MeterValuesRequest and TransactionEvent.
/// All sampled values in a MeterValue are sampled at the same point in time.
/// Used by: MeterValuesRequest, TransactionEventRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,
    /// Required. One or more measured values.
    pub sampled_value: Vec<SampledValueType>,
}

impl Default for MeterValueType {
    fn default() -> MeterValueType {
        Self {
            timestamp: Utc::now(),
            sampled_value: vec![],
        }
    }
}
#[typetag::serde]
impl OcppEntity for MeterValueType {
    /// Validates the fields of MeterValueType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("sampled_value", 1, usize::MAX, &self.sampled_value.iter());
        e.check_iter_member("sampled_value", self.sampled_value.iter());

        e.build("MeterValueType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let meter_value_type = MeterValueType {
            timestamp: Utc.timestamp_opt(1672531200, 0).unwrap(),
            sampled_value: vec![Default::default()],
        };
        assert!(meter_value_type.validate().is_ok());

        let meter_value_type_multiple = MeterValueType {
            timestamp: Utc.timestamp_opt(1672531200, 0).unwrap(),
            sampled_value: vec![Default::default(); 3],
        };
        assert!(meter_value_type_multiple.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_empty_sampled_value() {
        let meter_value_type = MeterValueType {
            timestamp: Utc.timestamp_opt(1672531200, 0).unwrap(),
            sampled_value: vec![], // Invalid length
        };
        let result = meter_value_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "sampled_value");
            } else {
                panic!("Expected FieldValidationError for 'sampled_value'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = MeterValueType {
            timestamp: Utc.timestamp_opt(1672531200, 0).unwrap(),
            sampled_value: vec![Default::default(); 2],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: MeterValueType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
