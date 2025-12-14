use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Contains status details, especially when an error has occurred.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    /// Required. A predefined code for the reason why the status is returned in this response. The string is case-insensitive.
    pub reason_code: String,
    /// Optional. Additional text to provide detailed information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl Default for StatusInfoType {
    fn default() -> StatusInfoType {
        Self {
            reason_code: "".to_string(),
            additional_info: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for StatusInfoType {
    /// Validates the fields of StatusInfoType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("reason_code", 0, 20, &self.reason_code.chars());
        if let Some(additional_info) = &self.additional_info {
            e.check_cardinality("additional_info", 0, 1024, &additional_info.chars());
        }

        e.build("StatusInfoType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let data = StatusInfoType {
            reason_code: "ValidCode".to_string(),
            additional_info: Some("Additional info.".to_string()),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let data = StatusInfoType {
            reason_code: "ValidCode".to_string(),
            additional_info: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_reason_code_too_long() {
        let mut data = StatusInfoType::default();
        data.reason_code = "a".repeat(21);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_failure_additional_info_too_long() {
        let mut data = StatusInfoType::default();
        data.reason_code = "ValidCode".to_string();
        data.additional_info = Some("a".repeat(1025));
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = StatusInfoType {
            reason_code: "ValidCode".to_string(),
            additional_info: Some("Additional info.".to_string()),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: StatusInfoType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
