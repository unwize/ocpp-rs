use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CostKindEnumType {
    /// Absolute value. Carbon Dioxide emissions, in grams per kWh.
    CarbonDioxideEmission,
    /// Relative value. Percentage of renewable generation within total generation.
    RelativePricePercentage,
    /// Relative value. Price per kWh, as percentage relative to the maximum price stated in any of all tariffs indicated to the EV.
    RenewableGenerationPercentage,
}

impl TryFrom<String> for CostKindEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "CarbonDioxideEmission" => Ok(CostKindEnumType::CarbonDioxideEmission),
            "RelativePricePercentage" => Ok(CostKindEnumType::RelativePricePercentage),
            "RenewableGenerationPercentage" => Ok(CostKindEnumType::RenewableGenerationPercentage),
            _ => Err(format!("'{}' is not a valid CostKindEnumType", s)),
        }
    }
}

impl Into<String> for CostKindEnumType {
    fn into(self) -> String {
        match self {
            CostKindEnumType::CarbonDioxideEmission => "CarbonDioxideEmission".to_string(),
            CostKindEnumType::RelativePricePercentage => "RelativePricePercentage".to_string(),
            CostKindEnumType::RenewableGenerationPercentage => "RenewableGenerationPercentage".to_string(),
        }
    }
}
