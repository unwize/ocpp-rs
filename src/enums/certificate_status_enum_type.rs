use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CertificateStatusEnumType {
    /// Certificate has not been revoked.
    Good,
    /// Certificate has been revoked.
    Revoked,
    /// Certificate is unknown.
    Unknown,
    /// The request to OCSP responder or CRL distribution point failed.
    Failed,
}

impl TryFrom<String> for CertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Good" => Ok(CertificateStatusEnumType::Good),
            "Revoked" => Ok(CertificateStatusEnumType::Revoked),
            "Unknown" => Ok(CertificateStatusEnumType::Unknown),
            "Failed" => Ok(CertificateStatusEnumType::Failed),
            _ => Err(format!("'{}' is not a valid CertificateStatusEnumType", s)),
        }
    }
}

impl Into<String> for CertificateStatusEnumType {
    fn into(self) -> String {
        match self {
            CertificateStatusEnumType::Good => "Good".to_string(),
            CertificateStatusEnumType::Revoked => "Revoked".to_string(),
            CertificateStatusEnumType::Unknown => "Unknown".to_string(),
            CertificateStatusEnumType::Failed => "Failed".to_string(),
        }
    }
}
