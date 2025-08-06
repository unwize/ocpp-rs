use serde::{Deserialize, Serialize};
use crate::errors::OcppError;
use crate::traits::OcppEntity;

/// An entry in schedule of the energy amount over time that EV is willing to discharge.
/// A negative value indicates the willingness to discharge under specific conditions,
/// a positive value indicates that the EV currently is not able to offer energy to discharge.
/// Used by: Common::EVPowerScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EVPowerScheduleEntryType {
    /// Required. The duration of this entry.
    pub duration: i32, // integer
    /// Required. Defines maximum amount of power for the duration of this EVPowerScheduleEntry to be discharged
    /// from the EV battery through EVSE power outlet. Negative values are used for discharging.
    pub power: f64, // decimal
}

impl OcppEntity for EVPowerScheduleEntryType {
    /// Validates the fields of EVPowerScheduleEntryType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

impl Default for EVPowerScheduleEntryType {
    fn default() -> EVPowerScheduleEntryType {
        EVPowerScheduleEntryType {
            duration: 0,
            power: 0.0,
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let entry = EVPowerScheduleEntryType {
            duration: 3600,
            power: -5000.0, // Example for discharging
        };

        let serialized = serde_json::to_string(&entry).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVPowerScheduleEntryType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let entry_discharging = EVPowerScheduleEntryType {
            duration: 1800,
            power: -2500.0,
        };
        assert!(entry_discharging.validate().is_ok());

        let entry_positive_power = EVPowerScheduleEntryType {
            duration: 60,
            power: 100.0, // EV not able to offer energy to discharge
        };
        assert!(entry_positive_power.validate().is_ok());

        let entry_zero_power = EVPowerScheduleEntryType {
            duration: 0,
            power: 0.0,
        };
        assert!(entry_zero_power.validate().is_ok());
    }
}
