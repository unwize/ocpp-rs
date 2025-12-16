use crate::enums::day_of_week_enum_type::DayOfWeekEnumType;
use crate::enums::evse_kind_enum_type::EvseKindEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::iso::rfc_3339::{validate_rfc3339_24hr_time, validate_rfc3339_date};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsType {
    /// Optional. Start time of day in local time. Format: HH:mm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,
    /// Optional. End time of day in local time. Format: HH:mm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_of_day: Option<String>,
    /// Optional. Day(s) of the week this tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,
    /// Optional. Start date in local time, format: YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,
    /// Optional. End date in local time, format: YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,
    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,
    /// Optional. Minimum consumed energy in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_energy: Option<f64>,
    /// Optional. Maximum consumed energy in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<f64>,
    /// Optional. Minimum current in Amperes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_current: Option<f64>,
    /// Optional. Maximum current in Amperes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_current: Option<f64>,
    /// Optional. Minimum power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<f64>,
    /// Optional. Maximum power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power: Option<f64>,
    /// Optional. Minimum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_time: Option<i32>,
    /// Optional. Maximum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i32>,
    /// Optional. Minimum duration in seconds in the charging period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_time: Option<i32>,
    /// Optional. Maximum duration in seconds in the charging period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charging_time: Option<i32>,
    /// Optional. Minimum duration in seconds in the idle period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_time: Option<i32>,
    /// Optional. Maximum duration in seconds in the idle period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_time: Option<i32>,
}

impl Default for TariffConditionsType {
    fn default() -> Self {
        Self {
            start_time_of_day: None,
            end_time_of_day: None,
            day_of_week: None,
            valid_from_date: None,
            valid_to_date: None,
            evse_kind: None,
            min_energy: None,
            max_energy: None,
            min_current: None,
            max_current: None,
            min_power: None,
            max_power: None,
            min_time: None,
            max_time: None,
            min_charging_time: None,
            max_charging_time: None,
            min_idle_time: None,
            max_idle_time: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for TariffConditionsType {
    /// Validates the fields of TariffConditionsType.
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
        if let Some(day_of_week) = &self.day_of_week {
            e.check_cardinality("dayOfWeek", 0, 7, &day_of_week.iter());
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
        // Negative value checks (fields are non-negative)
        if let Some(min_energy) = self.min_energy {
            e.check_bounds("min_energy", 0.0, f64::MAX, min_energy);
        }
        if let Some(max_energy) = self.max_energy {
            e.check_bounds("max_energy", 0.0, f64::MAX, max_energy);
        }
        if let Some(min_current) = self.min_current {
            e.check_bounds("min_current", 0.0, f64::MAX, min_current);
        }
        if let Some(max_current) = self.max_current {
            e.check_bounds("max_current", 0.0, f64::MAX, max_current);
        }
        if let Some(min_power) = self.min_power {
            e.check_bounds("min_power", 0.0, f64::MAX, min_power);
        }
        if let Some(max_power) = self.max_power {
            e.check_bounds("max_power", 0.0, f64::MAX, max_power);
        }
        if let Some(min_time) = self.min_time {
            e.check_bounds("min_time", 0, i32::MAX, min_time);
        }
        if let Some(max_time) = self.max_time {
            e.check_bounds("max_time", 0, i32::MAX, max_time);
        }
        if let Some(min_charging_time) = self.min_charging_time {
            e.check_bounds("min_charging_time", 0, i32::MAX, min_charging_time);
        }
        if let Some(max_charging_time) = self.max_charging_time {
            e.check_bounds("max_charging_time", 0, i32::MAX, max_charging_time);
        }
        if let Some(min_idle_time) = self.min_idle_time {
            e.check_bounds("min_idle_time", 0, i32::MAX, min_idle_time);
        }
        if let Some(max_idle_time) = self.max_idle_time {
            e.check_bounds("max_idle_time", 0, i32::MAX, max_idle_time);
        }

        e.build("TariffConditionsType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::day_of_week_enum_type::DayOfWeekEnumType;
    use crate::enums::evse_kind_enum_type::EvseKindEnumType;
    use serde_json;

    #[test]
    fn test_validate_success_minimal() {
        let data = TariffConditionsType::default();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_full() {
        let data = TariffConditionsType {
            start_time_of_day: Some("08:00".to_string()),
            end_time_of_day: Some("17:00".to_string()),
            day_of_week: Some(vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Friday]),
            valid_from_date: Some("2024-01-01".to_string()),
            valid_to_date: Some("2024-12-31".to_string()),
            evse_kind: Some(EvseKindEnumType::AC),
            min_energy: Some(10.0),
            max_energy: Some(100.0),
            min_current: Some(1.0),
            max_current: Some(32.0),
            min_power: Some(500.0),
            max_power: Some(22000.0),
            min_time: Some(300),
            max_time: Some(36000),
            min_charging_time: Some(60),
            max_charging_time: Some(18000),
            min_idle_time: Some(60),
            max_idle_time: Some(1800),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_invalid_time_format() {
        let mut data = TariffConditionsType::default();
        data.start_time_of_day = Some("25:00".to_string());
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_min_power() {
        let mut data = TariffConditionsType::default();
        data.min_power = Some(-1.0);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TariffConditionsType {
            start_time_of_day: Some("09:00".to_string()),
            end_time_of_day: None,
            day_of_week: None,
            valid_from_date: Some("2024-01-01".to_string()),
            valid_to_date: None,
            evse_kind: Some(EvseKindEnumType::AC),
            min_energy: Some(5.5),
            max_energy: None,
            min_current: None,
            max_current: None,
            min_power: None,
            max_power: None,
            min_time: None,
            max_time: None,
            min_charging_time: None,
            max_charging_time: None,
            min_idle_time: None,
            max_idle_time: None,
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffConditionsType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
