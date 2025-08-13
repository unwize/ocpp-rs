use serde::{Deserialize, Serialize};

use crate::errors::OcppError;
use crate::traits::OcppEntity;

/// Contains parameters for reactive power control.
/// Used by: Common:DERCurveType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReactivePowerParamsType {
    /// Optional. The nominal ac voltage (rms) adjustment to the voltage curve points for Volt-Var curves (percentage).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_ref: Option<f64>,
    /// Optional. Enable/disable autonomous VRef adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_v_ref_enable: Option<bool>,
    /// Optional. Adjustment range for VRef time constant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_v_ref_time_constant: Option<f64>,
}

impl Default for ReactivePowerParamsType {
    fn default() -> ReactivePowerParamsType {
        Self {
            v_ref: None,
            autonomous_v_ref_enable: None,
            autonomous_v_ref_time_constant: None,
        }
    }
}

impl OcppEntity for ReactivePowerParamsType {
    /// Validates the fields of ReactivePowerParamsType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let params = ReactivePowerParamsType {
            v_ref: Some(1.0),
            autonomous_v_ref_enable: Some(true),
            autonomous_v_ref_time_constant: Some(5.5),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let params = ReactivePowerParamsType {
            v_ref: None,
            autonomous_v_ref_enable: None,
            autonomous_v_ref_time_constant: None,
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = ReactivePowerParamsType {
            v_ref: Some(1.0),
            autonomous_v_ref_enable: Some(false),
            autonomous_v_ref_time_constant: Some(10.0),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: ReactivePowerParamsType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
