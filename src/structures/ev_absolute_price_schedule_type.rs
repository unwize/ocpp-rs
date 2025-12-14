use crate::errors::OcppError::FieldISOError;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::iso;
use crate::structures::ev_absolute_price_schedule_entry_type::EVAbsolutePriceScheduleEntryType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Price schedule of EV energy offer.
/// Used by: Common::EVEnergyOfferType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
    pub ev_absolute_price_schedule_entries: Vec<EVAbsolutePriceScheduleEntryType>,
}

impl Default for EVAbsolutePriceScheduleType {
    fn default() -> EVAbsolutePriceScheduleType {
        Self {
            time_anchor: Default::default(),
            currency: "".to_string(),
            price_algorithm: "".to_string(),
            ev_absolute_price_schedule_entries: vec![Default::default()],
        }
    }
}
#[typetag::serde]
impl OcppEntity for EVAbsolutePriceScheduleType {
    /// Validates the fields of EVAbsolutePriceScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Manually check if the `currency` field is in compliance with ISO-4217 and push a corresponding error if it is not.
        // Currently, there is no convenience function implemented for ISO errors.
        if !iso::iso_4217::CurrencyRegistry::new().is_valid_code(self.currency.as_str()) {
            e.push(
                FieldISOError {
                    value: self.currency.to_string(),
                    iso: "4217".to_string(),
                }
                .to_field_validation_error("currency"),
            );
        }

        e.check_cardinality("price_algorithm", 0, 2000, &self.price_algorithm.chars());
        e.check_cardinality(
            "ev_absolute_price_schedule_entries",
            1,
            1024,
            &self.ev_absolute_price_schedule_entries.iter(),
        );
        e.check_iter_member(
            "ev_absolute_price_schedule_entries",
            self.ev_absolute_price_schedule_entries.iter(),
        );

        e.build("EVAbsolutePriceScheduleType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "EUR".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:Power".to_string(),
            ev_absolute_price_schedule_entries: vec![Default::default()],
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVAbsolutePriceScheduleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schedule, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:PeakPower".to_string(),
            ev_absolute_price_schedule_entries: vec![Default::default()],
        };
        assert!(schedule.validate().is_ok());

        let schedule_max_lengths_and_cardinality = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),       // Max length
            price_algorithm: "a".repeat(2000), // Max length
            ev_absolute_price_schedule_entries: vec![Default::default(); 1024], // Max cardinality
        };
        println!("{:#?}", schedule_max_lengths_and_cardinality.validate());
        assert!(schedule_max_lengths_and_cardinality.validate().is_ok());
    }

    #[test]
    fn test_validation_price_algorithm_too_long() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "a".repeat(2001), // Invalid
            ev_absolute_price_schedule_entries: vec![Default::default(); 33],
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
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
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["ev_absolute_price_schedule_entries"]);
    }

    #[test]
    fn test_validation_ev_absolute_price_schedule_entries_too_many() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "USD".to_string(),
            price_algorithm: "urn:iso:15118:20:2022:Power".to_string(),
            ev_absolute_price_schedule_entries: vec![Default::default(); 1025], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["ev_absolute_price_schedule_entries"]);
    }

    #[test]
    fn test_validation_multiple_errors() {
        let schedule = EVAbsolutePriceScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            currency: "ABCD".to_string(),               // Invalid 1
            price_algorithm: "a".repeat(2001),          // Invalid 2
            ev_absolute_price_schedule_entries: vec![], // Invalid 3
        };
        let err = schedule.validate().unwrap_err();
        assert_num_field_errors(&err, 3);
        assert_invalid_fields(
            &err,
            &[
                "currency",
                "price_algorithm",
                "ev_absolute_price_schedule_entries",
            ],
        );
    }
}
