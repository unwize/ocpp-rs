use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::ev_power_schedule_entry_type::EVPowerScheduleEntryType;
use crate::traits::OcppEntity;

/// Schedule of EV energy offer.
/// Used by: Common::EVEnergyOfferType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EVPowerScheduleType {
    /// Required. The time that defines the starting point for the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,
    /// Required. List of EVPowerScheduleEntries.
    /// Cardinality 1..1024
    pub ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>
}

impl Default for EVPowerScheduleType {
    fn default() -> EVPowerScheduleType {
        Self {
            time_anchor: Default::default(),
            ev_power_schedule_entries: vec![],
        }
    }
}

impl OcppEntity for EVPowerScheduleType {
    /// Validates the fields of EVPowerScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_cardinality("ev_power_schedule_entries", 1, 1024, &self.ev_power_schedule_entries.iter());
        e.push_iter_member("ev_power_schedule_entries", self.ev_power_schedule_entries.iter());
        e.build("EVPowerScheduleType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_serialization_deserialization() {
        let schedule = EVPowerScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1,10, 0, 0).unwrap(),
            ev_power_schedule_entries: vec![Default::default()],
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVPowerScheduleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schedule, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let schedule = EVPowerScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1,10, 0, 0).unwrap(),
            ev_power_schedule_entries: vec![Default::default()],
        };
        assert!(schedule.validate().is_ok());

        let schedule_max_entries = EVPowerScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 0, 0, 0).unwrap(),
            ev_power_schedule_entries: vec![Default::default(); 1024], // Max cardinality
        };
        assert!(schedule_max_entries.validate().is_ok());
    }

    #[test]
    fn test_validation_ev_power_schedule_entries_empty() {
        let schedule = EVPowerScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            ev_power_schedule_entries: vec![], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(err, vec!["ev_power_schedule_entries".to_string()]);
    }

    #[test]
    fn test_validation_ev_power_schedule_entries_too_many() {
        let schedule = EVPowerScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            ev_power_schedule_entries: vec![Default::default(); 1025], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(err, vec!["ev_power_schedule_entries".to_string()]);

    }
}
