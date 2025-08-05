use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GetCertificateIdUseEnumType {
    /// Use for certificate of the ISO 15118 V2G Root.
    V2GRootCertificate,
    /// Use for certificate from an eMobility Service provider. To support PnC charging with contracts from service providers that not derived their certificates from the V2G root.
    MORootCertificate,
    /// Root certificate for verification of the CSMS certificate.
    CSMSRootCertificate,
    /// ISO 15118 V2G certificate chain (excluding the V2GRootCertificate).
    V2GCertificateChain,
    /// Root certificate for verification of the Manufacturer certificate.
    ManufacturerRootCertificate,
    /// OEM root certificate for 2-way TLS with EV.
    OEMRootCertificate,
}

impl TryFrom<String> for GetCertificateIdUseEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "V2GRootCertificate" => Ok(GetCertificateIdUseEnumType::V2GRootCertificate),
            "MORootCertificate" => Ok(GetCertificateIdUseEnumType::MORootCertificate),
            "CSMSRootCertificate" => Ok(GetCertificateIdUseEnumType::CSMSRootCertificate),
            "V2GCertificateChain" => Ok(GetCertificateIdUseEnumType::V2GCertificateChain),
            "ManufacturerRootCertificate" => Ok(GetCertificateIdUseEnumType::ManufacturerRootCertificate),
            "OEMRootCertificate" => Ok(GetCertificateIdUseEnumType::OEMRootCertificate),
            _ => Err(format!("'{}' is not a valid GetCertificateIdUseEnumType", s)),
        }
    }
}

impl Into<String> for GetCertificateIdUseEnumType {
    fn into(self) -> String {
        match self {
            GetCertificateIdUseEnumType::V2GRootCertificate => "V2GRootCertificate".to_string(),
            GetCertificateIdUseEnumType::MORootCertificate => "MORootCertificate".to_string(),
            GetCertificateIdUseEnumType::CSMSRootCertificate => "CSMSRootCertificate".to_string(),
            GetCertificateIdUseEnumType::V2GCertificateChain => "V2GCertificateChain".to_string(),
            GetCertificateIdUseEnumType::ManufacturerRootCertificate => "ManufacturerRootCertificate".to_string(),
            GetCertificateIdUseEnumType::OEMRootCertificate => "OEMRootCertificate".to_string(),
        }
    }
}
