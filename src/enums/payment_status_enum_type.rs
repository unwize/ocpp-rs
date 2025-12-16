use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// (2.1) Status of the settlement of an ad hoc payment.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentStatusEnumType {
    /// Settled successfully by the PSP.
    Settled,
    /// No billable part of the OCPP transaction, cancellation sent to the PSP.
    Canceled, // sic
    /// Rejected by the PSP.
    Rejected,
    /// Sent after the final attempt that fails due to communication problems.
    Failed,
}

impl fmt::Display for PaymentStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Settled => write!(f, "Settled"),
            Self::Canceled => write!(f, "Canceled"), // sic
            Self::Rejected => write!(f, "Rejected"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

impl From<PaymentStatusEnumType> for String {
    fn from(val: PaymentStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for PaymentStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Settled" => Ok(Self::Settled),
            "Canceled" => Ok(Self::Canceled), // sic
            "Rejected" => Ok(Self::Rejected),
            "Failed" => Ok(Self::Failed),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PaymentStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
