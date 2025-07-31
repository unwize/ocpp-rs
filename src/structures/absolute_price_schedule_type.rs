use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::structures::additional_selected_services_type::AdditionalSelectedServicesType;

/// Represents an absolute price schedule with timing and pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl AbsolutePriceScheduleType {

    /// Naively validate the values within the struct. Does not cross-validate against external
    /// data.
    pub fn is_valid(&self) -> bool {
        match &self.price_schedule_description {
            Some(d) => {if d.len() > 160 {return false}}
            None => {}
        }

        match self.language.len() <= 8 && self.language.len() > 0 {
            true => {
                // TODO: Check against ISO-639
            },
            false => {return false}
        }

        match self.price_algorithm.len() > 0 && self.price_algorithm.len() < 2000 {
            false => {return false}
            _ => {}
        }

        true
    }
}