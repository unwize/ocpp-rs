use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V2XFreqWattPointType {
    /// Required. Net frequency in Hz.
    pub frequency: f64,
    /// Required. Power in W to charge (positive) or discharge (negative) at the specified frequency.
    pub power: f64,
}

impl Default for V2XFreqWattPointType {
    fn default() -> V2XFreqWattPointType {
        Self {
            frequency: 0.0,
            power: 0.0,
        }
    }
}

impl OcppEntity for V2XFreqWattPointType {
    /// Validates the fields of V2XFreqWattPointType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Frequency is net frequency in Hz, should be non-negative.
        e.check_bounds("frequency", 0.0, f64::MAX, self.frequency);

        e.build("V2XFreqWattPointType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> V2XFreqWattPointType {
        V2XFreqWattPointType {
            frequency: 50.0,
            power: -1500.0,
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_frequency() {
        let mut data = create_test_instance();
        data.frequency = -1.0;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: V2XFreqWattPointType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
