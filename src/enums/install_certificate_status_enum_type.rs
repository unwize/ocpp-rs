use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum InstallCertificateStatusEnumType {
    /// The installation of the certificate succeeded.
    Accepted,
    /// The certificate is invalid and/or incorrect OR the CSO tries to install more certificates than allowed.
    Rejected,
    /// The certificate is valid and correct, but there is another reason the installation did not succeed.
    Failed,
}

impl TryFrom<String> for InstallCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(InstallCertificateStatusEnumType::Accepted),
            "Rejected" => Ok(InstallCertificateStatusEnumType::Rejected),
            "Failed" => Ok(InstallCertificateStatusEnumType::Failed),
            _ => Err(format!(
                "'{}' is not a valid InstallCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl From<InstallCertificateStatusEnumType> for String {
    fn from(val: InstallCertificateStatusEnumType) -> Self {
        match val {
            InstallCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            InstallCertificateStatusEnumType::Rejected => "Rejected".to_string(),
            InstallCertificateStatusEnumType::Failed => "Failed".to_string(),
        }
    }
}
