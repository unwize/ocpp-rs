use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents battery data.
/// Used by: BatterySwapRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatteryDataType {
    /// Required. Slot number where battery is inserted or removed.
    /// Constraints: 0 <= val
    pub evse_id: i32,
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
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("evse_id", 0, i32::MAX, self.evse_id);
        e.check_cardinality("serial_number", 0, 50, &self.serial_number.chars());
        e.check_bounds("soc", 0.0, 100.0, self.soc);
        e.check_bounds("soh", 0.0, 100.0, self.soh);

        // Validate vendor_info length if present
        if let Some(info) = &self.vendor_info {
            e.check_cardinality("vendor_info", 0, 500, &info.chars());
        }

        e.build("BatteryDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let battery_data = BatteryDataType {
            evse_id: 1,
            serial_number: "BAT-SN-12345".to_string(),
            soc: 85.5,
            soh: 92.1,
            production_date: Some(Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap()),
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
            evse_id: 0,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data.validate().is_ok());

        let battery_data_max_values = BatteryDataType {
            evse_id: 100,
            serial_number: "a".repeat(50),
            soc: 100.0,
            soh: 100.0,
            production_date: Some(Utc.with_ymd_and_hms(2025, 7, 31, 16, 0, 0).unwrap()),
            vendor_info: Some("b".repeat(500)),
        };
        assert!(battery_data_max_values.validate().is_ok());
    }

    #[test]
    fn test_validation_serial_number_too_long() {
        let battery_data = BatteryDataType {
            evse_id: 1,
            serial_number: "a".repeat(51), // Too long
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data.validate().is_err());
    }

    #[test]
    fn test_validation_soc_out_of_range() {
        let battery_data_low = BatteryDataType {
            evse_id: 1,
            serial_number: "SN001".to_string(),
            soc: -0.1, // Out of range
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data_low.validate().is_err());

        let battery_data_high = BatteryDataType {
            evse_id: 1,
            serial_number: "SN001".to_string(),
            soc: 100.1, // Out of range
            soh: 75.0,
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data_high.validate().is_err());
    }

    #[test]
    fn test_validation_soh_out_of_range() {
        let battery_data_low = BatteryDataType {
            evse_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: -0.1, // Out of range
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data_low.validate().is_err());

        let battery_data_high = BatteryDataType {
            evse_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 100.1, // Out of range
            production_date: None,
            vendor_info: None,
        };
        assert!(battery_data_high.validate().is_err());
    }

    #[test]
    fn test_validation_vendor_info_too_long() {
        let battery_data = BatteryDataType {
            evse_id: 1,
            serial_number: "SN001".to_string(),
            soc: 50.0,
            soh: 75.0,
            production_date: None,
            vendor_info: Some("a".repeat(501)), // Too long
        };
        assert!(battery_data.validate().is_err());
    }
}
