use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ComponentCriterionEnumType {
    /// Components that are active, i.e. having Active = 1.
    Active,
    /// Components that are available, i.e. having Available = 1.
    Available,
    /// Components that are enabled, i.e. having Enabled = 1.
    Enabled,
    /// Components that reported a problem, i.e. having Problem = 1.
    Problem,
}

impl TryFrom<String> for ComponentCriterionEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Active" => Ok(ComponentCriterionEnumType::Active),
            "Available" => Ok(ComponentCriterionEnumType::Available),
            "Enabled" => Ok(ComponentCriterionEnumType::Enabled),
            "Problem" => Ok(ComponentCriterionEnumType::Problem),
            _ => Err(format!("'{}' is not a valid ComponentCriterionEnumType", s)),
        }
    }
}

impl Into<String> for ComponentCriterionEnumType {
    fn into(self) -> String {
        match self {
            ComponentCriterionEnumType::Active => "Active".to_string(),
            ComponentCriterionEnumType::Available => "Available".to_string(),
            ComponentCriterionEnumType::Enabled => "Enabled".to_string(),
            ComponentCriterionEnumType::Problem => "Problem".to_string(),
        }
    }
}
