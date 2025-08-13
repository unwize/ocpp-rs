use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Type of cost: normal cost calculation, or limited to a min or max value.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TariffCostEnumType {
    /// Cost is result of normal cost calculation.
    NormalCost,
    /// Cost is the minimum cost for this tariff.
    MinCost,
    /// Cost is the maximum cost for this tariff.
    MaxCost,
}

impl fmt::Display for TariffCostEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NormalCost => write!(f, "NormalCost"),
            Self::MinCost => write!(f, "MinCost"),
            Self::MaxCost => write!(f, "MaxCost"),
        }
    }
}

impl Into<String> for TariffCostEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TariffCostEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "NormalCost" => Ok(Self::NormalCost),
            "MinCost" => Ok(Self::MinCost),
            "MaxCost" => Ok(Self::MaxCost),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffCostEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
