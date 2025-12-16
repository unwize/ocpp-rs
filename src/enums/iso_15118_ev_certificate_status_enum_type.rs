use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Iso15118EVCertificateStatusEnumType {
    /// exiResponse included. This is no indication whether the update was successful, just that the message was processed properly.
    #[default]
    Accepted,
    /// Processing of the message was not successful, no exiResponse included.
    Failed,
}

impl TryFrom<String> for Iso15118EVCertificateStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(Iso15118EVCertificateStatusEnumType::Accepted),
            "Failed" => Ok(Iso15118EVCertificateStatusEnumType::Failed),
            _ => Err(format!(
                "'{}' is not a valid Iso15118EVCertificateStatusEnumType",
                s
            )),
        }
    }
}

impl From<Iso15118EVCertificateStatusEnumType> for String {
    fn from(val: Iso15118EVCertificateStatusEnumType) -> Self {
        match val {
            Iso15118EVCertificateStatusEnumType::Accepted => "Accepted".to_string(),
            Iso15118EVCertificateStatusEnumType::Failed => "Failed".to_string(),
        }
    }
}
