use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringBaseEnumType {
    /// Activate all pre-configured monitors while leaving custom monitors intact, including those that overrule a pre-configured monitor.
    All,
    /// (Re)activate the default monitors of the charging station and remove all custom monitors.
    FactoryDefault,
    /// Removes all custom monitors and disables all pre-configured monitors.
    HardWiredOnly,
}

impl fmt::Display for MonitoringBaseEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::FactoryDefault => write!(f, "FactoryDefault"),
            Self::HardWiredOnly => write!(f, "HardWiredOnly"),
        }
    }
}

impl From<MonitoringBaseEnumType> for String {
    fn from(val: MonitoringBaseEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for MonitoringBaseEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "All" => Ok(Self::All),
            "FactoryDefault" => Ok(Self::FactoryDefault),
            "HardWiredOnly" => Ok(Self::HardWiredOnly),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MonitoringBaseEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
