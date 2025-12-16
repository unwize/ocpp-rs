use crate::enums::certificate_action_enum_type::CertificateActionEnumType;
use crate::enums::iso_15118_ev_certificate_status_enum_type::Iso15118EVCertificateStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.23. Get15118EVCertificate
pub struct Get15118EVCertificate;

impl OcppMessage for Get15118EVCertificate {
    type Request = Get15118EVCertificateRequest;
    type Response = Get15118EVCertificateResponse;
}

/// 1.23.1. Get15118EVCertificateRequest
/// This contains the field definition of the Get15118EVCertificateRequest PDU sent by the Charging Station to the CSMS if an ISO 15118 vehicle selects the service Certificate Installation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    /// Required. Schema version currently used for the 15118 session between EV and Charging Station.
    pub iso15118_schema_version: String,
    /// Required. Defines whether certificate needs to be installed or updated.
    pub action: CertificateActionEnumType,
    /// Required. (2.1) Raw CertificateInstallationReq request from EV, Base64 encoded.
    pub exi_request: String,
    /// Optional. (2.1) Absent during ISO 15118-2 session. Required during ISO 15118-20 session. Maximum number of contracts that EV wants to install.
    pub maximum_contract_certificate_chains: Option<i32>,
    /// Optional. (2.1) Absent during ISO 15118-2 session. Optional during ISO 15118-20 session. List of email IDs for which contract certificates must be requested first, in case there are more certificates than allowed by `maximumContractCertificateChains`.
    pub prioritized_em_aids: Option<Vec<String>>,
}
#[typetag::serde]
impl OcppEntity for Get15118EVCertificateRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality(
            "iso15118_schema_version",
            0,
            50,
            &self.iso15118_schema_version.chars(),
        );
        b.check_cardinality("exi_request", 0, 11000, &self.exi_request.chars());

        if let Some(max_chains) = self.maximum_contract_certificate_chains {
            b.check_bounds(
                "maximum_contract_certificate_chains",
                0,
                i32::MAX,
                max_chains,
            );
        }

        if let Some(prioritized_em_aids) = &self.prioritized_em_aids {
            b.check_cardinality("prioritized_em_aids", 0, 8, &prioritized_em_aids.iter());
            for em_aid in prioritized_em_aids {
                b.check_cardinality("prioritized_em_aids", 0, 255, &em_aid.chars());
            }
        }

        b.build("Get15118EVCertificateRequest")
    }
}

#[typetag::serde]
impl OcppRequest for Get15118EVCertificateRequest {
    fn get_message_type(&self) -> String {
        String::from("Get15118EVCertificate")
    }
}

/// 1.23.2. Get15118EVCertificateResponse
/// Response message from CSMS to Charging Station containing the status and optionally new certificate.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    /// Required. Indicates whether the message was processed properly.
    pub status: Iso15118EVCertificateStatusEnumType,
    /// Required. (2/1) Raw CertificateInstallationRes response for the EV, Base64 encoded.
    pub exi_response: String,
    /// Optional. (2.1) Number of contracts that can be retrieved with additional requests.
    pub remaining_contracts: Option<i32>,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for Get15118EVCertificateResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation needed.
        b.check_cardinality("exi_response", 0, 17000, &self.exi_response.chars());

        if let Some(remaining_contracts) = self.remaining_contracts {
            b.check_bounds("remaining_contracts", 0, i32::MAX, remaining_contracts);
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("Get15118EVCertificateResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = Get15118EVCertificate::request();
        let resp = Get15118EVCertificate::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = Get15118EVCertificateRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: Get15118EVCertificateRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = Get15118EVCertificateResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: Get15118EVCertificateResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(Get15118EVCertificate::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(Get15118EVCertificate::response().validate().is_ok());
    }
}
