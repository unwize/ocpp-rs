use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// The physical system where an Electrical Vehicle (EV) can be charged.
/// Used by: BootNotificationRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingStationType {
    /// Optional. Vendor-specific device identifier.
    /// String length: 0..25
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Required. Defines the model of the device.
    /// String length: 0..20
    pub model: String,
    /// Required. Identifies the vendor (not necessarily in a unique manner).
    /// String length: 0..50
    pub vendor_name: String,
    /// Optional. This contains the firmware version of the Charging Station.
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// Optional. Defines the functional parameters of a communication link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>, // TODO: Implement ModemType
}

impl ChargingStationType {
    /// Validates the fields of ChargingStationType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate serial_number length
        if let Some(serial_num) = &self.serial_number {
            if serial_num.len() > 25 {
                errors.push(OcppError::FieldValidationError {
                    field: "serial_number".to_string(),
                    source: vec![OcppError::FieldCardinalityError {
                        cardinality: serial_num.len(),
                        lower: 0,
                        upper: 25,
                    }],
                });
            }
        }

        // Validate model length
        if self.model.len() > 20 {
            errors.push(OcppError::FieldValidationError {
                field: "model".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.model.len(),
                    lower: 0,
                    upper: 20,
                }],
            });
        }

        // Validate vendor_name length
        if self.vendor_name.len() > 50 {
            errors.push(OcppError::FieldValidationError {
                field: "vendor_name".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.vendor_name.len(),
                    lower: 0,
                    upper: 50,
                }],
            });
        }

        // Validate firmware_version length
        if let Some(fw_version) = &self.firmware_version {
            if fw_version.len() > 50 {
                errors.push(OcppError::FieldValidationError {
                    field: "firmware_version".to_string(),
                    source: vec![OcppError::FieldCardinalityError {
                        cardinality: fw_version.len(),
                        lower: 0,
                        upper: 50,
                    }],
                });
            }
        }

        // TODO:
        // No validation for 'modem' without its type definition.
        // If ModemType had its own validate method, you would call it here:
        // if let Some(modem_data) = &self.modem {
        //     if let Err(e) = modem_data.validate() {
        //         errors.push(OcppError::FieldValidationError {
        //             field: "modem".to_string(),
        //             source: vec![e],
        //         });
        //     }
        // }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ChargingStationType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::assert_invalid_fields;
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let cs = ChargingStationType {
            serial_number: Some("ABC123XYZ".to_string()),
            model: "ModelX".to_string(),
            vendor_name: "VendorCorp".to_string(),
            firmware_version: Some("1.0.0".to_string()),
            modem: Some("modem_placeholder".to_string()), // Placeholder
        };

        let serialized = serde_json::to_string(&cs).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingStationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cs, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cs_minimal = ChargingStationType {
            serial_number: None,
            model: "BasicModel".to_string(),
            vendor_name: "MyVendor".to_string(),
            firmware_version: None,
            modem: None,
        };
        assert!(cs_minimal.validate().is_ok());

        let cs_full_lengths = ChargingStationType {
            serial_number: Some("a".repeat(25)),
            model: "b".repeat(20),
            vendor_name: "c".repeat(50),
            firmware_version: Some("d".repeat(50)),
            modem: None,
        };
        assert!(cs_full_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_serial_number_too_long() {
        let cs = ChargingStationType {
            serial_number: Some("a".repeat(26)), // Invalid
            model: "ModelX".to_string(),
            vendor_name: "VendorCorp".to_string(),
            firmware_version: None,
            modem: None,
        };
        let err = cs.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "serial_number");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_model_too_long() {
        let cs = ChargingStationType {
            serial_number: None,
            model: "a".repeat(21), // Invalid
            vendor_name: "VendorCorp".to_string(),
            firmware_version: None,
            modem: None,
        };
        let err = cs.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "model");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_vendor_name_too_long() {
        let cs = ChargingStationType {
            serial_number: None,
            model: "ModelX".to_string(),
            vendor_name: "a".repeat(51), // Invalid
            firmware_version: None,
            modem: None,
        };
        let err = cs.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "vendor_name");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_firmware_version_too_long() {
        let cs = ChargingStationType {
            serial_number: None,
            model: "ModelX".to_string(),
            vendor_name: "VendorCorp".to_string(),
            firmware_version: Some("a".repeat(51)), // Invalid
            modem: None,
        };
        let err = cs.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "firmware_version");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let cs = ChargingStationType {
            serial_number: Some("a".repeat(26)), // Invalid 1
            model: "b".repeat(21), // Invalid 2
            vendor_name: "c".repeat(51), // Invalid 3
            firmware_version: Some("d".repeat(51)), // Invalid 4
            modem: None,
        };
        let err = cs.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "serial_number".to_string(),
            "model".to_string(),
            "vendor_name".to_string(),
            "firmware_version".to_string(),
        ]);
    }
}
