use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CertificateSignedStatusEnumType {
    /// Signed certificate is valid.
    Accepted,
    /// Signed certificate is invalid or requestId is unknown.
    Rejected,
}

impl TryFrom<String> for CertificateSignedStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(CertificateSignedStatusEnumType::Accepted),
            "Rejected" => Ok(CertificateSignedStatusEnumType::Rejected),
            _ => Err(format!(
                "'{}' is not a valid CertificateSignedStatusEnumType",
                s
            )),
        }
    }
}

impl Into<String> for CertificateSignedStatusEnumType {
    fn into(self) -> String {
        match self {
            CertificateSignedStatusEnumType::Accepted => "Accepted".to_string(),
            CertificateSignedStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
