use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum CertificateActionEnumType {
    /// Install the provided certificate.
    #[default]
    Install,
    /// Update the provided certificate.
    Update,
}

impl TryFrom<String> for CertificateActionEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Install" => Ok(CertificateActionEnumType::Install),
            "Update" => Ok(CertificateActionEnumType::Update),
            _ => Err(format!("'{}' is not a valid CertificateActionEnumType", s)),
        }
    }
}

impl From<CertificateActionEnumType> for String {
    fn from(val: CertificateActionEnumType) -> Self {
        match val {
            CertificateActionEnumType::Install => "Install".to_string(),
            CertificateActionEnumType::Update => "Update".to_string(),
        }
    }
}
