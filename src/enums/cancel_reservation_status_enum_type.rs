use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CancelReservationStatusEnumType {
    /// Reservation for the identifier has been canceled.
    Accepted,
    /// Reservation could not be canceled, because there is no reservation active for the identifier.
    Rejected,
}

impl TryFrom<String> for CancelReservationStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(CancelReservationStatusEnumType::Accepted),
            "Rejected" => Ok(CancelReservationStatusEnumType::Rejected),
            _ => Err(format!("'{}' is not a valid CancelReservationStatusEnumType", s)),
        }
    }
}

impl Into<String> for CancelReservationStatusEnumType {
    fn into(self) -> String {
        match self {
            CancelReservationStatusEnumType::Accepted => "Accepted".to_string(),
            CancelReservationStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
