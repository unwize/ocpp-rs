use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::errors::OcppError;
use crate::iso;
use crate::structures::ev_absolute_price_schedule_entry_type::EVAbsolutePriceScheduleEntryType;

/// Price schedule of EV energy offer.
/// Used by: Common::EVEnergyOfferType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EVAbsolutePriceScheduleType {
    /// Required. Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,
    /// Required. Currency code according to ISO 4217.
    /// String length: 0..3
    pub currency: String,
    /// Required. ISO 15118-20 URN of price algorithm: Power, PeakPower, StackedEnergy.
    /// String length: 0..2000
    pub price_algorithm: String,
    /// Required. Schedule of prices for which EV is willing to discharge.
    /// Cardinality 1..1024
    pub ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>
}

impl EVAbsolutePriceScheduleType {
    /// Validates the fields of EVAbsolutePriceScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate currency length
        if !iso::iso_4217::CurrencyRegistry::new().is_valid_code(self.currency.as_str()) {
            errors.push(OcppError::FieldValidationError {
                field: "currency".to_string(),
                source: vec![OcppError::FieldValueError {
                    value: "currency".to_string(),
                }],
            });
        }

        // Validate price_algorithm length
        if self.price_algorithm.len() > 2000 {
            errors.push(OcppError::FieldValidationError {
                field: "price_algorithm".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.price_algorithm.len(),
                    lower: 0,
                    upper: 2000,
                }],
            });
        }

        // Validate ev_absolute_price_schedule_entries cardinality
        if self.ev_absolute_price_schedule_entries.is_empty() || self.ev_absolute_price_schedule_entries.len() > 1024 {
            errors.push(OcppError::FieldValidationError {
                field: "ev_absolute_price_schedule_entries".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.ev_absolute_price_schedule_entries.len(),
                    lower: 1,
                    upper: 1024,
                }],
            });
        }
        for (i, entry) in self.ev_absolute_price_schedule_entries.iter().enumerate() {
            if let Err(e) = entry.validate() {
                errors.push(OcppError::FieldValidationError {
                    field: format!("ev_absolute_price_schedule_entries[{}]", i),
                    source: vec![e],
                });
            }
        }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EVAbsolutePriceScheduleType".to_string(),
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
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "EUR".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:Power".to_string(),
            ev_absolute_price_schedule_entries: vec![], // TODO:  Placeholder
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVAbsolutePriceScheduleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schedule, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1,10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:PeakPower".to_string(),
            ev_absolute_price_schedule_entries: vec![],
        };
        assert!(schedule.validate().is_ok());

        let schedule_max_lengths_and_cardinality = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "ABC".to_string(), // Max length
            price_algorithm: "a".repeat(2000), // Max length
            ev_absolute_price_schedule_entries: vec![Default::default(); 1024], // Max cardinality
        };
        assert!(schedule_max_lengths_and_cardinality.validate().is_ok());
    }


    #[test]
    fn test_validation_price_algorithm_too_long() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "a".repeat(2001), // Invalid
            ev_absolute_price_schedule_entries: vec![],
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "price_algorithm");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_ev_absolute_price_schedule_entries_empty() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:Power".to_string(),
            ev_absolute_price_schedule_entries: vec![], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_absolute_price_schedule_entries");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_ev_absolute_price_schedule_entries_too_many() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.ymd(2025, 8, 1).and_hms(10, 0, 0),
            currency: "USD".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:Power".to_string(),
            ev_absolute_price_schedule_entries: vec![Default::default(); 1025], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_absolute_price_schedule_entries");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "ABCD".to_string(), // Invalid 1
            price_algorithm: "a".repeat(2001), // Invalid 2
            ev_absolute_price_schedule_entries: vec![], // Invalid 3
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(err, vec!["currency".to_string(), "price_algorithm".to_string(), "ev_absolute_price_schedule_entries".to_string()]);
    }
}
