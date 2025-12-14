use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::message_content_type::MessageContentType;
use crate::structures::price_type::PriceType;
use crate::structures::tariff_energy_type::TariffEnergyType;
use crate::structures::tariff_fixed_type::TariffFixedType;
use crate::structures::tariff_time_type::TariffTimeType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffType {
    /// Required. Unique identifier of this tariff.
    pub tariff_id: String,
    /// Required. Currency code according to ISO 4217.
    pub currency: String,
    /// Optional. Time when this tariff becomes active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. List of multi-language tariff information texts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<MessageContentType>>,
    /// Optional. Energy tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<TariffEnergyType>,
    /// Optional. Charging time tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_time: Option<TariffTimeType>,
    /// Optional. Idle time tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time: Option<TariffTimeType>,
    /// Optional. Fixed fee tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<TariffFixedType>,
    /// Optional. Minimum cost for a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cost: Option<PriceType>,
    /// Optional. Maximum cost for a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost: Option<PriceType>,
    /// Optional. Reservation time tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<TariffTimeType>,
    /// Optional. Fixed fee for a reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_fixed: Option<TariffFixedType>,
}

impl Default for TariffType {
    fn default() -> TariffType {
        Self {
            tariff_id: "".to_string(),
            currency: "".to_string(),
            valid_from: None,
            description: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            fixed_fee: None,
            min_cost: None,
            max_cost: None,
            reservation_time: None,
            reservation_fixed: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for TariffType {
    /// Validates the fields of TariffType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("tariff_id", 0, 60, &self.tariff_id.chars());
        e.check_cardinality("currency", 0, 3, &self.currency.chars());

        if let Some(description) = &self.description {
            e.check_cardinality("description", 0, 10, &description.iter());
            e.check_iter_member("description", description.iter());
        }

        if let Some(energy) = &self.energy {
            e.check_member("energy", energy);
        }

        if let Some(charging_time) = &self.charging_time {
            e.check_member("charging_time", charging_time);
        }

        if let Some(idle_time) = &self.idle_time {
            e.check_member("idle_time", idle_time);
        }

        if let Some(fixed_fee) = &self.fixed_fee {
            e.check_member("fixed_fee", fixed_fee);
        }

        if let Some(min_cost) = &self.min_cost {
            e.check_member("min_cost", min_cost);
        }

        if let Some(max_cost) = &self.max_cost {
            e.check_member("max_cost", max_cost);
        }

        if let Some(reservation_time) = &self.reservation_time {
            e.check_member("reservation_time", reservation_time);
        }

        if let Some(reservation_fixed) = &self.reservation_fixed {
            e.check_member("reservation_fixed", reservation_fixed);
        }

        e.build("TariffType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TariffType {
        TariffType {
            tariff_id: "T123".to_string(),
            currency: "USD".to_string(),
            valid_from: Some(Utc::now()),
            description: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            fixed_fee: None,
            min_cost: None,
            max_cost: None,
            reservation_time: None,
            reservation_fixed: None,
        }
    }

    #[test]
    fn test_validate_success_minimal() {
        let mut data = create_test_instance();
        data.valid_from = None;
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_maximal() {
        let data = TariffType {
            tariff_id: "T123".to_string(),
            currency: "EUR".to_string(),
            valid_from: Some(Utc::now()),
            description: Some(vec![MessageContentType::default(); 5]),
            energy: Some(TariffEnergyType::default()),
            charging_time: Some(TariffTimeType::default()),
            idle_time: Some(TariffTimeType::default()),
            fixed_fee: Some(TariffFixedType::default()),
            min_cost: Some(PriceType::default()),
            max_cost: Some(PriceType::default()),
            reservation_time: Some(TariffTimeType::default()),
            reservation_fixed: Some(TariffFixedType::default()),
        };

        if let Err(e) = data.validate() {
            match e {
                OcppError::StructureValidationError { related, .. } => {
                    println!("{:#?}", related);
                }

                _ => {}
            }
        }

        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_tariff_id_too_long() {
        let mut data = create_test_instance();
        data.tariff_id = "a".repeat(61);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_currency_too_long() {
        let mut data = create_test_instance();
        data.currency = "ABCD".to_string();
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_description_cardinality_max() {
        let mut data = create_test_instance();
        data.description = Some(vec![MessageContentType::default(); 11]);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
