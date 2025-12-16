use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CertificateSigningUseEnumType {
    /// Client side certificate used by the Charging Station to connect the the CSMS.
    ChargingStationCertificate,
    /// Use for certificate for ISO 15118-2 connections. This means that the certificate should be derived from the V2G root.
    V2GCertificate,
    /// Use for certificate for ISO 15118-20 connections. This means that the certificate should be derived from the V2G root.
    V2G20Certificate,
}

impl TryFrom<String> for CertificateSigningUseEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "ChargingStationCertificate" => {
                Ok(CertificateSigningUseEnumType::ChargingStationCertificate)
            }
            "V2GCertificate" => Ok(CertificateSigningUseEnumType::V2GCertificate),
            "V2G20Certificate" => Ok(CertificateSigningUseEnumType::V2G20Certificate),
            _ => Err(format!(
                "'{}' is not a valid CertificateSigningUseEnumType",
                s
            )),
        }
    }
}

impl From<CertificateSigningUseEnumType> for String {
    fn from(val: CertificateSigningUseEnumType) -> Self {
        match val {
            CertificateSigningUseEnumType::ChargingStationCertificate => {
                "ChargingStationCertificate".to_string()
            }
            CertificateSigningUseEnumType::V2GCertificate => "V2GCertificate".to_string(),
            CertificateSigningUseEnumType::V2G20Certificate => "V2G20Certificate".to_string(),
        }
    }
}
