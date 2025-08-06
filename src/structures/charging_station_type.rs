use serde::{Deserialize, Serialize};
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

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

impl OcppEntity for ChargingStationType {
    /// Validates the fields of ChargingStationType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(serial_number) = &self.serial_number {
            e.check_cardinality("serial_number", 0, 25, &serial_number.chars());
        }  
        
        e.check_cardinality("model", 0, 20, &self.model.chars());
        e.check_cardinality("vendor_name", 0, 50, &self.vendor_name.chars());
        
        if let Some(firmware_version) = &self.firmware_version {
            e.check_cardinality("firmware_version", 0, 50, &firmware_version.chars());
        }
       
        
        if let Some(modem) = &self.modem {
            e.check_member("modem", modem);
        }

        e.build("ChargingStationType")
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
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
