use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::errors::{validate_string_length, OcppError};
use crate::errors::OcppError::StructureValidationError;
use crate::iso::iso_4217::CurrencyRegistry;
use crate::structures::additional_selected_services_type::AdditionalSelectedServicesType;
use crate::traits::OcppEntity;

/// Represents an absolute price schedule with timing and pricing information
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePriceScheduleType {
    /// Required. Starting point of price schedule.
    pub time_anchor: DateTime<Utc>,

    /// Required. Unique ID of price schedule
    pub price_schedule_id: u32, // integer, 0 <= val

    /// Optional. Description of the price schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_schedule_description: Option<String>, // string[0..160]

    /// Required. Currency according to ISO 4217.
    pub currency: String, // string[0..3]

    /// Required. String that indicates what language is used for
    /// the human-readable strings in the price schedule. Based on ISO 639.
    pub language: String, // string[0..8]

    /// Required. A string in URN notation which shall uniquely
    /// identify an algorithm that defines how to compute an
    /// energy fee sum for a specific power profile based on the
    /// EnergyFee information from the PriceRule elements.
    pub price_algorithm: String, // string[0..2000]

    /// Required. A set of pricing rules for parking and energy costs.
    pub price_rule_stacks: PriceRuleStackType,

    /// Optional. Optional. Describes the applicable tax rule(s) for this price schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rules: Option<TaxRuleType>,

    ///  Optional. A set of prices for optional services (e.g. valet, carwash).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_selected_services: Option<AdditionalSelectedServicesType>,

    /// A set of overstay rules that allows for escalation of charges after the overstay is triggered.
    overstay_rule_list: Option<OverstayRuleListTypeOptional>,

    /// Optional. Minimum amount to be billed for the overall charging session (e.g. including energy, parking, and overstay).
    minimum_cost: Option<u32>,

    ///  Optional. Maximum amount to be billed for the overall charging session (e.g. including energy, parking, and overstay).
    maximum_cost: Option<u32>
}

impl OcppEntity for AbsolutePriceScheduleType {
    /// Naively validate the values within the struct. Does not cross-validate against external
    /// data.
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate str len for price_schedule_description
        if let Some(price_schedule_description) = &self.price_schedule_description {
            if let(Err(e)) = validate_string_length(price_schedule_description, 0, 160) {
                errors.push(e);
            }
        }

        // Validate ISO compliance for currency
        if !CurrencyRegistry::new().is_valid_code(self.currency.as_str()) {
            errors.push(OcppError::FieldISOError {
                value: "currency".to_string(),
                iso: "4217".to_string(),
            })
        }

        if let Err(e) = validate_string_length(self.language.as_str(), 0, 8) {
            errors.push(e);
        }

        if let Err(e) = validate_string_length(self.price_algorithm.as_str(), 0, 2000) {
            errors.push(e);
        }

        if let Err(e) = self.price_rule_stacks.validate() {
            errors.push(e);
        }

        if let Some(tax_rules) = &self.tax_rules {
            if let Err(e) = tax_rules.validate() {
                errors.push(e);
            }
        }

        if let Some(additional_selected_services) = &self.additional_selected_services {
            if let Err(e) = additional_selected_services.validate() {
                errors.push(e);
            }
        }

        if errors.is_empty() {Ok(())} else {
            Err(StructureValidationError {
                structure: "AbsolutePriceScheduleType".to_string(),
                source: errors
            })
        }


    }
}