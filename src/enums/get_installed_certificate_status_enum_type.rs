use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum GetInstalledCertificateStatusEnumType {
    /// Normal successful completion (no errors).
    #[default]
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
            _ => Err(format!(
                "'{}' is not a valid GetInstalledCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl From<GetInstalledCertificateStatusEnumType> for String {
    fn from(val: GetInstalledCertificateStatusEnumType) -> Self {
        match val {
            GetInstalledCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            GetInstalledCertificateStatusEnumType::NotFound => "NotFound".to_string(),
        }
    }
}
