use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration of VPN Types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VPNEnumType {
    #[serde(rename = "IKEv2")]
    Ikev2,
    #[serde(rename = "IPSec")]
    Ipsec,
    #[serde(rename = "L2TP")]
    L2Tp,
    #[serde(rename = "PPTP")]
    Pptp,
}

impl fmt::Display for VPNEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ikev2 => write!(f, "IKEv2"),
            Self::Ipsec => write!(f, "IPSec"),
            Self::L2Tp => write!(f, "L2TP"),
            Self::Pptp => write!(f, "PPTP"),
        }
    }
}

impl Into<String> for VPNEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for VPNEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "IKEv2" => Ok(Self::Ikev2),
            "IPSec" => Ok(Self::Ipsec),
            "L2TP" => Ok(Self::L2Tp),
            "PPTP" => Ok(Self::Pptp),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "VPNEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}