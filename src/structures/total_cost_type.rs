use crate::enums::tariff_cost_enum_type::TariffCostEnumType;
use crate::errors::OcppError::FieldISOError;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::iso::iso_4217::CurrencyRegistry;
use crate::structures::price_type::PriceType;
use crate::structures::total_price_type::TotalPriceType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalCostType {
    /// Required. Currency of the costs in ISO 4217 Code.
    pub currency: String,
    /// Required. Type of cost: normal or the minimum or maximum cost.
    pub type_of_cost: TariffCostEnumType,
    /// Optional. Total sum of all flat fees in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<PriceType>,
    /// Optional. Total sum of all the cost of all the energy used, in the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<PriceType>,
    /// Optional. Total sum of all the cost related to duration of charging during this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_time: Option<PriceType>,
    /// Optional. Total sum of all the cost related to idle time of this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time: Option<PriceType>,
    /// Optional. Total sum of all time-based cost related to reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<PriceType>,
    /// Required. Total sum of associated cost elements for fixed, energy, chargingTime, idleTime and reservation.
    pub total: TotalPriceType,
    /// Optional. Sum of fixed cost related to reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_fixed: Option<PriceType>,
}

impl Default for TotalCostType {
    fn default() -> TotalCostType {
        Self {
            currency: "USD".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            total: Default::default(),
            reservation_fixed: None,
        }
    }
}

impl OcppEntity for TotalCostType {
    /// Validates the fields of TotalCostType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if !CurrencyRegistry::new().is_valid_code(&self.currency) {
            e.push(
                FieldISOError {
                    value: self.currency.to_string(),
                    iso: "4217".to_string(),
                }
                .to_field_validation_error("currency"),
            );
        }

        if let Some(fixed) = &self.fixed {
            e.check_member("fixed", fixed);
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
        if let Some(reservation_time) = &self.reservation_time {
            e.check_member("reservation_time", reservation_time);
        }
        if let Some(reservation_fixed) = &self.reservation_fixed {
            e.check_member("reservation_fixed", reservation_fixed);
        }

        e.check_member("total", &self.total);

        e.build("TotalCostType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = TotalCostType::default();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_currency_too_long() {
        let mut data = TotalCostType::default();
        data.currency = "ABCD".to_string();
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TotalCostType::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TotalCostType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
