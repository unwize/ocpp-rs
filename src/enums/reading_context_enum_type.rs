use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Values of the context field.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReadingContextEnumType {
    /// Value taken at start of interruption.
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    /// Value taken when resuming after interruption.
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    /// Value for any other situations.
    Other,
    /// Value taken at clock aligned interval.
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    /// Value taken as periodic sample relative to start time of transaction.
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    /// Value taken at start of transaction.
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    /// Value taken at end of transaction.
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    /// Value taken in response to TriggerMessageRequest.
    Trigger,
}

impl fmt::Display for ReadingContextEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InterruptionBegin => write!(f, "Interruption.Begin"),
            Self::InterruptionEnd => write!(f, "Interruption.End"),
            Self::Other => write!(f, "Other"),
            Self::SampleClock => write!(f, "Sample.Clock"),
            Self::SamplePeriodic => write!(f, "Sample.Periodic"),
            Self::TransactionBegin => write!(f, "Transaction.Begin"),
            Self::TransactionEnd => write!(f, "Transaction.End"),
            Self::Trigger => write!(f, "Trigger"),
        }
    }
}

impl Into<String> for ReadingContextEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ReadingContextEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Interruption.Begin" => Ok(Self::InterruptionBegin),
            "Interruption.End" => Ok(Self::InterruptionEnd),
            "Other" => Ok(Self::Other),
            "Sample.Clock" => Ok(Self::SampleClock),
            "Sample.Periodic" => Ok(Self::SamplePeriodic),
            "Transaction.Begin" => Ok(Self::TransactionBegin),
            "Transaction.End" => Ok(Self::TransactionEnd),
            "Trigger" => Ok(Self::Trigger),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ReadingContextEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
