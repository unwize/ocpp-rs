use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Defines parameters required for initiating and maintaining wireless communication with other devices.
/// Used by: BootNotificationRequest.ChargingStationType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// Optional. This contains the ICCID of the modem's SIM card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    /// Optional. This contains the IMSI of the modem's SIM card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}

impl Default for ModemType {
    fn default() -> Self {
        Self {
            iccid: None,
            imsi: None,
        }
    }
}

impl OcppEntity for ModemType {
    /// Validates the fields of ModemType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(iccid) = &self.iccid {
            e.check_cardinality("iccid", 0, 20, &iccid.chars());
        }

        if let Some(imsi) = &self.imsi {
            e.check_cardinality("imsi", 0, 20, &imsi.chars());
        }

        e.build("ModemType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let modem_type = ModemType {
            iccid: Some("12345678901234567890".to_string()),
            imsi: Some("09876543210987654321".to_string()),
        };
        assert!(modem_type.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let modem_type = ModemType {
            iccid: None,
            imsi: None,
        };
        assert!(modem_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_iccid_length() {
        let modem_type = ModemType {
            iccid: Some("a".repeat(21)), // Invalid length
            imsi: None,
        };
        let result = modem_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "iccid");
            } else {
                panic!("Expected FieldValidationError for 'iccid'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_imsi_length() {
        let modem_type = ModemType {
            iccid: None,
            imsi: Some("b".repeat(21)), // Invalid length
        };
        let result = modem_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "imsi");
            } else {
                panic!("Expected FieldValidationError for 'imsi'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = ModemType {
            iccid: Some("some_iccid_value".to_string()),
            imsi: Some("some_imsi_value".to_string()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: ModemType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);

        let original_struct_minimal = ModemType {
            iccid: None,
            imsi: None,
        };
        let serialized_minimal = serde_json::to_string(&original_struct_minimal).unwrap();
        let deserialized_minimal: ModemType = serde_json::from_str(&serialized_minimal).unwrap();
        assert_eq!(original_struct_minimal, deserialized_minimal);
    }
}
