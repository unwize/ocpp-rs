use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CertificateStatusSourceEnumType {
    /// Checked in a certificate revocation list.
    Crl,
    /// Checked via OCSP request.
    Ocsp,
}

impl TryFrom<String> for CertificateStatusSourceEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "CRL" => Ok(CertificateStatusSourceEnumType::Crl),
            "OCSP" => Ok(CertificateStatusSourceEnumType::Ocsp),
            _ => Err(format!(
                "'{}' is not a valid CertificateStatusSourceEnumType",
                s
            )),
        }
    }
}

impl From<CertificateStatusSourceEnumType> for String {
    fn from(val: CertificateStatusSourceEnumType) -> Self {
        match val {
            CertificateStatusSourceEnumType::Crl => "CRL".to_string(),
            CertificateStatusSourceEnumType::Ocsp => "OCSP".to_string(),
        }
    }
}
