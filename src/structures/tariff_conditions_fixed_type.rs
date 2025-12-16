use crate::enums::day_of_week_enum_type::DayOfWeekEnumType;
use crate::enums::evse_kind_enum_type::EvseKindEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::iso::rfc_3339::{validate_rfc3339_24hr_time, validate_rfc3339_date};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct TariffConditionsFixedType {
    /// Optional. Start time of day in local time. Format: HH:mm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,
    /// Optional. End time of day in local time. Format: HH:mm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_of_day: Option<String>,
    /// Optional. Days of the week this tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,
    /// Optional. Start date for which this tariff is valid. Format: YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,
    /// Optional. End date for which this tariff is valid. Format: YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,
    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,
    /// Optional. For which payment brand this (adhoc) tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_brand: Option<String>,
    /// Optional. Type of ad-hoc payment, e.g. CC, Debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_recognition: Option<String>,
}

#[typetag::serde]
impl OcppEntity for TariffConditionsFixedType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(start_time_of_day) = &self.start_time_of_day
            && let Err(err) = validate_rfc3339_24hr_time(start_time_of_day)
        {
            e.push(err.to_field_validation_error("start_time_of_day"));
        }
        if let Some(end_time_of_day) = &self.end_time_of_day
            && let Err(err) = validate_rfc3339_24hr_time(end_time_of_day)
        {
            e.push(err.to_field_validation_error("end_time_of_day"));
        }
        if let Some(days) = &self.day_of_week {
            e.check_cardinality("dayOfWeek", 0, 7, &days.iter());
        }
        if let Some(valid_from_date) = &self.valid_from_date
            && let Err(err) = validate_rfc3339_date(valid_from_date)
        {
            e.push(err.to_field_validation_error("valid_from_date"));
        }
        if let Some(valid_to_date) = &self.valid_to_date
            && let Err(err) = validate_rfc3339_date(valid_to_date)
        {
            e.push(err.to_field_validation_error("valid_to_date"));
        }
        if let Some(payment_brand) = &self.payment_brand {
            e.check_cardinality("payment_brand", 0, 20, &payment_brand.chars());
        }
        if let Some(payment_recognition) = &self.payment_recognition {
            e.check_cardinality("payment_recognition", 0, 20, &payment_recognition.chars());
        }

        e.build("TariffConditionsFixedType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = TariffConditionsFixedType::default();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_time_format() {
        let mut data = TariffConditionsFixedType::default();
        data.start_time_of_day = Some("10:30".to_string());
        data.end_time_of_day = Some("22:00".to_string());
        assert!(data.validate().is_ok());

        data.start_time_of_day = Some("25:00".to_string());
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_date_format() {
        let mut data = TariffConditionsFixedType::default();
        data.valid_from_date = Some("2023-12-25".to_string());
        data.valid_to_date = Some("2024-01-01".to_string());
        assert!(data.validate().is_ok());

        data.valid_from_date = Some("2023/12/25".to_string());
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_payment_brand_length() {
        let mut data = TariffConditionsFixedType::default();
        data.payment_brand = Some("validbrand".to_string());
        assert!(data.validate().is_ok());

        data.payment_brand = Some("a".repeat(21));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TariffConditionsFixedType {
            start_time_of_day: Some("09:00".to_string()),
            end_time_of_day: Some("17:00".to_string()),
            day_of_week: Some(vec![]),
            valid_from_date: Some("2024-01-15".to_string()),
            valid_to_date: Some("2024-03-31".to_string()),
            evse_kind: Some(EvseKindEnumType::AC),
            payment_brand: Some("BrandX".to_string()),
            payment_recognition: Some("CC".to_string()),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffConditionsFixedType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
