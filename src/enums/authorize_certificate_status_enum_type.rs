use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AuthorizeCertificateStatusEnumType {
    /// Positive response
    Accepted,
    /// <not used>
    SignatureError,
    /// If the contract certificate in the AuthorizeRequest is expired.
    CertificateExpired,
    /// If the Charging Station or CSMS determine (via a CRL or OCSP response) that the contract certificate in the AuthorizeRequest is marked as revoked.
    CertificateRevoked,
    /// <not used>
    NoCertificateAvailable,
    /// If the contract certificate contained in the AuthorizeRequest message is not valid.
    CertChainError,
    /// If the EMAID provided by EVCC is invalid, unknown, expired or blocked.
    ContractCancelled,
}

impl TryFrom<String> for AuthorizeCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(AuthorizeCertificateStatusEnumType::Accepted),
            "SignatureError" => Ok(AuthorizeCertificateStatusEnumType::SignatureError),
            "CertificateExpired" => Ok(AuthorizeCertificateStatusEnumType::CertificateExpired),
            "CertificateRevoked" => Ok(AuthorizeCertificateStatusEnumType::CertificateRevoked),
            "NoCertificateAvailable" => {
                Ok(AuthorizeCertificateStatusEnumType::NoCertificateAvailable)
            }
            "CertChainError" => Ok(AuthorizeCertificateStatusEnumType::CertChainError),
            "ContractCancelled" => Ok(AuthorizeCertificateStatusEnumType::ContractCancelled),
            _ => Err(format!(
                "'{}' is not a valid AuthorizeCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl Into<String> for AuthorizeCertificateStatusEnumType {
    fn into(self) -> String {
        match self {
            AuthorizeCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            AuthorizeCertificateStatusEnumType::SignatureError => "SignatureError".to_string(),
            AuthorizeCertificateStatusEnumType::CertificateExpired => {
                "CertificateExpired".to_string()
            }
            AuthorizeCertificateStatusEnumType::CertificateRevoked => {
                "CertificateRevoked".to_string()
            }
            AuthorizeCertificateStatusEnumType::NoCertificateAvailable => {
                "NoCertificateAvailable".to_string()
            }
            AuthorizeCertificateStatusEnumType::CertChainError => "CertChainError".to_string(),
            AuthorizeCertificateStatusEnumType::ContractCancelled => {
                "ContractCancelled".to_string()
            }
        }
    }
}
