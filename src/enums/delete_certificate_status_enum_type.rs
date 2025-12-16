use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum DeleteCertificateStatusEnumType {
    /// Normal successful completion (no errors).
    #[default]
    Accepted,
    /// The Charging Station either failed to remove the certificate or rejected the request. A Charging Station may reject the request to prevent the deletion of a certificate, if it is the last one of its certificate type.
    Failed,
    /// Requested resource not found.
    NotFound,
}

impl TryFrom<String> for DeleteCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(DeleteCertificateStatusEnumType::Accepted),
            "Failed" => Ok(DeleteCertificateStatusEnumType::Failed),
            "NotFound" => Ok(DeleteCertificateStatusEnumType::NotFound),
            _ => Err(format!(
                "'{}' is not a valid DeleteCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl From<DeleteCertificateStatusEnumType> for String {
    fn from(val: DeleteCertificateStatusEnumType) -> Self {
        match val {
            DeleteCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            DeleteCertificateStatusEnumType::Failed => "Failed".to_string(),
            DeleteCertificateStatusEnumType::NotFound => "NotFound".to_string(),
        }
    }
}
