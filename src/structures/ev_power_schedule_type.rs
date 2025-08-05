use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::errors::OcppError;
use crate::structures::ev_power_schedule_entry_type::EVPowerScheduleEntryType;

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

impl EVPowerScheduleType {
    /// Validates the fields of EVPowerScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate ev_power_schedule_entries cardinality
        if self.ev_power_schedule_entries.is_empty() || self.ev_power_schedule_entries.len() > 1024 {
            errors.push(OcppError::FieldValidationError {
                field: "ev_power_schedule_entries".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.ev_power_schedule_entries.len(),
                    lower: 1,
                    upper: 1024,
                }],
            });
        }
        for (i, entry) in self.ev_power_schedule_entries.iter().enumerate() {
            if let Err(e) = entry.validate() {
                errors.push(OcppError::FieldValidationError {
                    field: format!("ev_power_schedule_entries[{}]", i),
                    source: vec![e],
                });
            }
        }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EVPowerScheduleType".to_string(),
                source: errors,
            })
        }
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
