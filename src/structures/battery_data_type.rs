use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::errors::OcppError;
use crate::traits::OcppEntity;

/// Represents battery data.
/// Used by: BatterySwapRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatteryDataType {
    /// Required. Slot number where battery is inserted or removed.
    /// Constraints: 0 <= val
    pub ev_se_id: i32,
    /// Required. Serial number of battery.
    /// String length: 0..50
    pub serial_number: String,
    /// Required. State of charge.
    /// Constraints: 0 <= val <= 100
    pub soc: f64,
    /// Required. State of health.
    /// Constraints: 0 <= val <= 100
    pub soh: f64,
    /// Optional. Production date of battery.
    pub production_date: Option<DateTime<Utc>>,
    /// Optional. Vendor-specific info from battery in undefined format.
    /// String length: 0..500
    pub vendor_info: Option<String>,
}

impl OcppEntity for BatteryDataType {
    /// Validates the fields of BatteryDataType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(self: &Self) -> Result<(), OcppError> {
        // Validate ev_se_id
        if self.ev_se_id < 0 {
            // println!("Validation failed: ev_se_id must be non-negative.");
            return false;
        }

        // Validate serial_number length
        if self.serial_number.len() > 50 {
            // println!("Validation failed: serial_number length exceeds 50.");
            return false;
        }

        // Validate soc
        if self.soc < 0.0 || self.soc > 100.0 {
            // println!("Validation failed: soc must be between 0 and 100.");
            return false;
        }

        // Validate soh
        if self.soh < 0.0 || self.soh > 100.0 {
            // println!("Validation failed: soh must be between 0 and 100.");
            return false;
        }

        // Validate vendor_info length if present
        if let Some(info) = &self.vendor_info {
            if info.len() > 500 {
                // println!("Validation failed: vendor_info length exceeds 500.");
                return false;
            }
        }

        true
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let battery_data = BatteryDataType {
            ev_se_id: 1,
            serial_number: "BAT-SN-12345".to_string(),
            soc: 85.5,
            soh: 92.1,
            production_date: Some(Utc.ymd(2023, 1, 15).and_hms(10, 0, 0)),
            vendor_info: Some("Vendor specific data here.".to_string()),
        };

        let serialized = serde_json::to_string(&battery_data).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: BatteryDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(battery_data, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let battery_data = BatteryDataType {
            ev_se_id: 0,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data.validate());

        let battery_data_max_values = BatteryDataType {
            ev_se_id: 100,
            serial_number: "a".repeat(50),
            soc: 100.0,
            soh: 100.0,
            production_date: Some(Utc.ymd(2025, 7, 31).and_hms(16, 0, 0)),
            vendor_info: Some("b".repeat(500)),
        };
        assert!(battery_data_max_values.validate());
    }

    #[test]
    fn test_validation_invalid_ev_se_id() {
        let battery_data = BatteryDataType {
            ev_se_id: -1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data.validate());
    }

    #[test]
    fn test_validation_serial_number_too_long() {
        let battery_data = BatteryDataType {
            ev_se_id: 1,
            serial_number: "a".repeat(51), // Too long
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data.validate());
    }

    #[test]
    fn test_validation_soc_out_of_range() {
        let battery_data_low = BatteryDataType {
            ev_se_id: 1,
            serial_number: "SN001".to_string(),
            soc: -0.1, // Out of range
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data_low.validate());

        let battery_data_high = BatteryDataType {
            ev_se_id: 1,
            serial_number: "SN001".to_string(),
            soc: 100.1, // Out of range
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data_high.validate());
    }

    #[test]
    fn test_validation_soh_out_of_range() {
        let battery_data_low = BatteryDataType {
            ev_se_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: -0.1, // Out of range
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data_low.validate());

        let battery_data_high = BatteryDataType {
            ev_se_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 100.1, // Out of range
            production_date: None,
            vendor_info: None,
        };
        assert!(!battery_data_high.validate());
    }

    #[test]
    fn test_validation_vendor_info_too_long() {
        let battery_data = BatteryDataType {
            ev_se_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: Some("a".repeat(501)), // Too long
        };
        assert!(!battery_data.validate());
    }
}
