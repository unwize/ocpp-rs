use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GetInstalledCertificateStatusEnumType {
    /// Normal successful completion (no errors).
    Accepted,
    /// Requested resource not found.
    NotFound,
}

impl TryFrom<String> for GetInstalledCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GetInstalledCertificateStatusEnumType::Accepted),
            "NotFound" => Ok(GetInstalledCertificateStatusEnumType::NotFound),
            _ => Err(format!("'{}' is not a valid GetInstalledCertificateStatusEnumType", s)),
        }
    }
}

impl Into<String> for GetInstalledCertificateStatusEnumType {
    fn into(self) -> String {
        match self {
            GetInstalledCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            GetInstalledCertificateStatusEnumType::NotFound => "NotFound".to_string(),
        }
    }
}
