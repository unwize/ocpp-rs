use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GetCertificateStatusEnumType {
    /// Successfully retrieved the OCSP certificate status.
    Accepted,
    /// Failed to retrieve the OCSP certificate status.
    Failed,
}

impl TryFrom<String> for GetCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GetCertificateStatusEnumType::Accepted),
            "Failed" => Ok(GetCertificateStatusEnumType::Failed),
            _ => Err(format!("'{}' is not a valid GetCertificateStatusEnumType", s)),
        }
    }
}

impl Into<String> for GetCertificateStatusEnumType {
    fn into(self) -> String {
        match self {
            GetCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            GetCertificateStatusEnumType::Failed => "Failed".to_string(),
        }
    }
}
