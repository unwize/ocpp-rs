use crate::enums::customer_information_status_enum_type::CustomerInformationStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::certificate_hash_data_type::CertificateHashDataType;
use crate::structures::id_token_type::IdTokenType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.19. CustomerInformation
pub struct CustomerInformation;

impl OcppMessage for CustomerInformation {
    type Request = CustomerInformationRequest;
    type Response = CustomerInformationResponse;
}

/// 1.19.1. CustomerInformationRequest
/// This contains the field definition of the CustomerInformationRequest PDU sent by the CSMS to the Charging Station. With this request the CSMS can send the current cost of a transaction to a Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    /// Required. The Id of the request.
    pub request_id: i32,
    /// Required. Flag indicating whether the Charging Station should return `NotifyCustomerInformationRequest` messages containing information about the customer referred to.
    pub report: bool,
    /// Required. Flag indicating whether the Charging Station should clear all information about the customer referred to.
    pub clear: bool,
    /// Optional. A (e.g. vendor specific) identifier of the customer this request refers to. This field contains a custom identifier other than `IdToken` and `Certificate`. One of the possible identifiers (`customerIdentifier`, `idToken` or `customerCertificate`) should be in the request message.
    pub customer_identifier: Option<String>,
    /// Optional. The `idToken` of the customer this request refers to. One of the possible identifiers (`customerIdentifier`, `idToken` or `customerCertificate`) should be in the request message.
    pub id_token: Option<IdTokenType>,
    /// Optional. The Certificate of the customer this request refers to. One of the possible identifiers (`customerIdentifier`, `idToken` or `customerCertificate`) should be in the request message.
    pub customer_certificate: Option<CertificateHashDataType>,
}
#[typetag::serde]
impl OcppEntity for CustomerInformationRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_bounds("request_id", 0, i32::MAX, self.request_id);

        if let Some(customer_identifier) = &self.customer_identifier {
            b.check_cardinality("customer_identifier", 0, 64, &customer_identifier.chars());
        }

        if let Some(id_token) = &self.id_token {
            b.check_member("id_token", id_token);
        }

        if let Some(customer_certificate) = &self.customer_certificate {
            b.check_member("customer_certificate", customer_certificate);
        }

        b.build("CustomerInformationRequest")
    }
}

impl OcppRequest for CustomerInformationRequest {
    fn get_message_type(&self) -> String {
        String::from("CustomerInformation")
    }
}

/// 1.19.2. CustomerInformationResponse
/// This contains the field definition of the CustomerInformationResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    /// Required. Indicates whether the request was accepted.
    pub status: CustomerInformationStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for CustomerInformationResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("CustomerInformationResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = CustomerInformation::request();
        let resp = CustomerInformation::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = CustomerInformationRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: CustomerInformationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = CustomerInformationResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: CustomerInformationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(CustomerInformation::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(CustomerInformation::response().validate().is_ok());
    }
}
