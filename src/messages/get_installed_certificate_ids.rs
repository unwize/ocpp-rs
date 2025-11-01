use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;
use crate::enums::get_installed_certificate_status_enum_type::GetInstalledCertificateStatusEnumType;
use crate::structures::certificate_hash_data_chain_type::CertificateHashDataChainType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.31. GetInstalledCertificateIds
pub struct GetInstalledCertificateIds;

impl OcppMessage for GetInstalledCertificateIds {
    type Request = GetInstalledCertificateIdsRequest;
    type Response = GetInstalledCertificateIdsResponse;
}

/// 1.31.1. GetInstalledCertificateIdsRequest
/// Used by the CSMS to request an overview of the installed certificates on a Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    /// Optional. Indicates the type of certificates requested. When omitted, all certificate types are requested.
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,
}

impl OcppEntity for GetInstalledCertificateIdsRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(certificate_types) = &self.certificate_type {
            b.check_cardinality("certificate_type", 0, usize::MAX, &certificate_types.iter());
        }

        b.build("GetInstalledCertificateIdsRequest")
    }
}

/// 1.31.2. GetInstalledCertificateIdsResponse
/// Response to a GetInstalledCertificateIdsRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    /// Required. Charging Station indicates if it can process the request.
    pub status: GetInstalledCertificateStatusEnumType,
    /// Optional. The Charging Station includes the Certificate information for each available certificate.
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for GetInstalledCertificateIdsResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation needed.

        if let Some(chains) = &self.certificate_hash_data_chain {
            // Cardinality: 0..*
            b.check_cardinality("certificate_hash_data_chain", 0, usize::MAX, &chains.iter());
            // Individual member validation
            b.check_iter_member("certificate_hash_data_chain", chains.iter());
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetInstalledCertificateIdsResponse")
    }
}