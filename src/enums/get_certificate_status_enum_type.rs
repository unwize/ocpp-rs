use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum GetCertificateStatusEnumType {
    /// Successfully retrieved the OCSP certificate status.
    #[default]
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
            _ => Err(format!(
                "'{}' is not a valid GetCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl From<GetCertificateStatusEnumType> for String {
    fn from(val: GetCertificateStatusEnumType) -> Self {
        match val {
            GetCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            GetCertificateStatusEnumType::Failed => "Failed".to_string(),
        }
    }
}
