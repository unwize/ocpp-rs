use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::price_level_schedule_entry_type::PriceLevelScheduleEntryType;
use crate::traits::OcppEntity;

/// The PriceLevelScheduleType is modeled after the same type that is defined in ISO 15118-20, such that if it is supplied by an EMSP as a signed EXI message, the conversion from EXI to JSON (in OCPP) and back to EXI (for ISO 15118-20) does not change the digest and therefore does not invalidate the signature.
/// Used by: Common:ChargingScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleType {
    /// Required. Starting point of this price schedule.
    pub time_anchor: DateTime<Utc>,
    /// Required. Unique ID of this price schedule.
    pub price_schedule_id: i32,
    /// Optional. Description of the price schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_schedule_description: Option<String>,
    /// Required. Defines the overall number of distinct price level elements used across all PriceLevelSchedules.
    pub number_of_price_levels: i32,
    /// Required. List of entries of the schedule.
    pub price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
}

impl OcppEntity for PriceLevelScheduleType {
    /// Validates the fields of PriceLevelScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("price_schedule_id", 0, i32::MAX, self.price_schedule_id);

        if let Some(desc) = &self.price_schedule_description {
            e.check_cardinality("price_schedule_description", 0, 32, &desc.chars());
        }

        e.check_bounds(
            "number_of_price_levels",
            0,
            i32::MAX,
            self.number_of_price_levels,
        );
        e.check_cardinality(
            "price_level_schedule_entries",
            1,
            100,
            &self.price_level_schedule_entries.iter(),
        );
        e.check_iter_member(
            "price_level_schedule_entries",
            self.price_level_schedule_entries.iter(),
        );

        e.build("PriceLevelScheduleType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: Some("Winter schedule".to_string()),
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![Default::default()],
        };
        assert!(schedule.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: None,
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![Default::default()],
        };
        assert!(schedule.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_description_length() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: Some("a".repeat(33)), // Invalid length
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![Default::default()],
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_schedule_description"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_price_schedule_id_negative() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: -1, // Invalid: must be >= 0
            price_schedule_description: None,
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![Default::default()],
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_schedule_id"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_number_of_price_levels_negative() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: None,
            number_of_price_levels: -1, // Invalid: must be >= 0
            price_level_schedule_entries: vec![Default::default()],
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(&err, &["number_of_price_levels"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_price_level_schedule_entries_too_few() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: None,
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![], // Invalid: min 1
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_level_schedule_entries"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_price_level_schedule_entries_too_many() {
        let schedule = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 1,
            price_schedule_description: None,
            number_of_price_levels: 10,
            price_level_schedule_entries: vec![Default::default(); 101], // Invalid: max 100
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_level_schedule_entries"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PriceLevelScheduleType {
            time_anchor: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            price_schedule_id: 42,
            price_schedule_description: Some("Test Schedule".to_string()),
            number_of_price_levels: 5,
            price_level_schedule_entries: vec![Default::default(), Default::default()],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PriceLevelScheduleType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
