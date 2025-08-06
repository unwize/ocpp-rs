use crate::errors::{OcppError, StructureValidationBuilder};
use crate::iso::iso_4217::CurrencyRegistry;
use crate::structures::additional_selected_services_type::AdditionalSelectedServicesType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an absolute price schedule with timing and pricing information
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePriceScheduleType {
    /// Required. Starting point of price schedule.
    pub time_anchor: DateTime<Utc>,

    /// Required. Unique ID of price schedule
    pub price_schedule_id: i32, // integer, 0 <= val

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
    overstay_rule_list: Option<OverstayRuleListType>,

    /// Optional. Minimum amount to be billed for the overall charging session (e.g. including energy, parking, and overstay).
    minimum_cost: Option<u32>,

    ///  Optional. Maximum amount to be billed for the overall charging session (e.g. including energy, parking, and overstay).
    maximum_cost: Option<u32>
}

impl OcppEntity for AbsolutePriceScheduleType {
    /// Naively validate the values within the struct. Does not cross-validate against external
    /// data.
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        
        e.check_bounds("price_schedule_id", 0, i32::MAX, self.price_schedule_id);
        
        if let Some(price_schedule_description) = &self.price_schedule_description {
            e.check_cardinality("price_schedule_description", 0, 160, &price_schedule_description.chars());
        }

        // Validate ISO compliance for currency
        if !CurrencyRegistry::new().is_valid_code(self.currency.as_str()) {
            e.push(OcppError::FieldISOError {
                value: "currency".to_string(),
                iso: "4217".to_string(),
            }.to_field_validation_error("currency"));
        }

        e.check_cardinality("language", 0, 8, self.language.as_ref());
        e.check_cardinality("price_algorithm", 0, 2000, self.price_algorithm.as_ref());
        e.push_member("price_rule_stacks", &self.price_rule_stacks);

        if let Some(tax_rules) = &self.tax_rules {
            e.push_member("tax_rules", &tax_rules);
        }

        if let Some(additional_selected_services) = &self.additional_selected_services {
            e.push_member("additional_selected_services", additional_selected_services);
        }

        e.build("AbsolutePriceScheduleType")
    }
}