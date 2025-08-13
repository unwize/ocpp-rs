use serde::{Deserialize, Serialize};

use crate::errors::OcppError;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V2XSignalWattPointType {
    /// Required. Signal value from an AFRRSignalRequest.
    pub signal: i32,
    /// Required. Power in W to charge (positive) or discharge (negative) at a specified frequency.
    pub power: f64,
}

impl Default for V2XSignalWattPointType {
    fn default() -> V2XSignalWattPointType {
        Self {
            signal: 0,
            power: 0.0,
        }
    }
}

impl OcppEntity for V2XSignalWattPointType {
    /// Validates the fields of V2XSignalWattPointType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> V2XSignalWattPointType {
        V2XSignalWattPointType {
            signal: 10,
            power: 2500.5,
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: V2XSignalWattPointType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
