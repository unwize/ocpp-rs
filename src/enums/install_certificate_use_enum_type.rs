use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum InstallCertificateUseEnumType {
    /// Use for certificate of the ISO 15118 V2G Root. A V2G Charging Station Certificate MUST be derived from one of the installed V2GRootCertificate certificates.
    V2GRootCertificate,
    /// Use for certificate from an eMobility Service provider. To support PnC charging with contracts from service providers that not derived their certificates from the V2G root.
    MORootCertificate,
    /// Root certificate for verification of the Manufacturer certificate.
    ManufacturerRootCertificate,
    /// Root certificate, used by the CA to sign the CSMS and Charging Station certificate.
    CSMSRootCertificate,
    /// OEM root certificate for 2-way TLS with EV.
    OEMRootCertificate,
}

impl TryFrom<String> for InstallCertificateUseEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "V2GRootCertificate" => Ok(InstallCertificateUseEnumType::V2GRootCertificate),
            "MORootCertificate" => Ok(InstallCertificateUseEnumType::MORootCertificate),
            "ManufacturerRootCertificate" => {
                Ok(InstallCertificateUseEnumType::ManufacturerRootCertificate)
            }
            "CSMSRootCertificate" => Ok(InstallCertificateUseEnumType::CSMSRootCertificate),
            "OEMRootCertificate" => Ok(InstallCertificateUseEnumType::OEMRootCertificate),
            _ => Err(format!(
                "'{}' is not a valid InstallCertificateUseEnumType",
                s
            )),
        }
    }
}

impl From<InstallCertificateUseEnumType> for String {
    fn from(val: InstallCertificateUseEnumType) -> Self {
        match val {
            InstallCertificateUseEnumType::V2GRootCertificate => "V2GRootCertificate".to_string(),
            InstallCertificateUseEnumType::MORootCertificate => "MORootCertificate".to_string(),
            InstallCertificateUseEnumType::ManufacturerRootCertificate => {
                "ManufacturerRootCertificate".to_string()
            }
            InstallCertificateUseEnumType::CSMSRootCertificate => "CSMSRootCertificate".to_string(),
            InstallCertificateUseEnumType::OEMRootCertificate => "OEMRootCertificate".to_string(),
        }
    }
}
