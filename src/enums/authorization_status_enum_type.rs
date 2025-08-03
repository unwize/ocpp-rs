use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AuthorizationStatusEnumType {
    /// Identifier is allowed for charging.
    Accepted,
    /// Identifier has been blocked. Not allowed for charging.
    Blocked,
    /// Identifier is already involved in another transaction and multiple transactions are not allowed. (Only relevant for the response to a TransactionEventRequest(eventType=Started).)
    ConcurrentTx,
    /// Identifier has expired. Not allowed for charging.
    Expired,
    /// Identifier is invalid. Not allowed for charging.
    Invalid,
    /// Identifier is valid, but EV Driver doesn't have enough credit to start charging. Not allowed for charging.
    NoCredit,
    /// Identifier is valid, but not allowed to charge at this type of EVSE.
    NotAllowedTypeEVS,
    /// Identifier is valid, but not allowed to charge at this location.
    NotAtThisLocation,
    /// Identifier is valid, but not allowed to charge at this location at this time.
    NotAtThisTime,
    /// Identifier is unknown. Not allowed for charging.
    Unknown,
}

impl TryFrom<String> for AuthorizationStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(AuthorizationStatusEnumType::Accepted),
            "Blocked" => Ok(AuthorizationStatusEnumType::Blocked),
            "ConcurrentTx" => Ok(AuthorizationStatusEnumType::ConcurrentTx),
            "Expired" => Ok(AuthorizationStatusEnumType::Expired),
            "Invalid" => Ok(AuthorizationStatusEnumType::Invalid),
            "NoCredit" => Ok(AuthorizationStatusEnumType::NoCredit),
            "NotAllowedTypeEVS" => Ok(AuthorizationStatusEnumType::NotAllowedTypeEVS),
            "NotAtThisLocation" => Ok(AuthorizationStatusEnumType::NotAtThisLocation),
            "NotAtThisTime" => Ok(AuthorizationStatusEnumType::NotAtThisTime),
            "Unknown" => Ok(AuthorizationStatusEnumType::Unknown),
            _ => Err(format!("'{}' is not a valid AuthorizationStatusEnumType", s)),
        }
    }
}

impl Into<String> for AuthorizationStatusEnumType {
    fn into(self) -> String {
        match self {
            AuthorizationStatusEnumType::Accepted => "Accepted".to_string(),
            AuthorizationStatusEnumType::Blocked => "Blocked".to_string(),
            AuthorizationStatusEnumType::ConcurrentTx => "ConcurrentTx".to_string(),
            AuthorizationStatusEnumType::Expired => "Expired".to_string(),
            AuthorizationStatusEnumType::Invalid => "Invalid".to_string(),
            AuthorizationStatusEnumType::NoCredit => "NoCredit".to_string(),
            AuthorizationStatusEnumType::NotAllowedTypeEVS => "NotAllowedTypeEVS".to_string(),
            AuthorizationStatusEnumType::NotAtThisLocation => "NotAtThisLocation".to_string(),
            AuthorizationStatusEnumType::NotAtThisTime => "NotAtThisTime".to_string(),
            AuthorizationStatusEnumType::Unknown => "Unknown".to_string(),
        }
    }
}
