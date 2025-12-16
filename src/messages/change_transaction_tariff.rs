use crate::enums::tariff_change_status_enum_type::TariffChangeStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::structures::tariff_type::TariffType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.9. ChangeTransactionTariff
pub struct ChangeTransactionTariff;

impl OcppMessage for ChangeTransactionTariff {
    type Request = ChangeTransactionTariffRequest;
    type Response = ChangeTransactionTariffResponse;
}

/// 1.9.1. ChangeTransactionTariffRequest
/// This contains the field definition of the ChangeTransactionTariffRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffRequest {
    /// Required. Transaction id for new tariff.
    pub transaction_id: String,
    /// Required. New tariff to use for transaction.
    pub tariff: TariffType,
}
#[typetag::serde]
impl OcppEntity for ChangeTransactionTariffRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality("transaction_id", 0, 36, &self.transaction_id.chars());
        b.check_member("tariff", &self.tariff);

        b.build("ChangeTransactionTariffRequest")
    }
}

impl OcppRequest for ChangeTransactionTariffRequest {
    fn get_message_type(&self) -> String {
        String::from("ChangeTransactionTariff")
    }
}

/// 1.9.2. ChangeTransactionTariffResponse
/// This contains the field definition of the ChangeTransactionTariffResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffResponse {
    /// Required. Status of the operation
    pub status: TariffChangeStatusEnumType,
    /// Optional. Detailed status information
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for ChangeTransactionTariffResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, so no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ChangeTransactionTariffResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ChangeTransactionTariff::request();
        let resp = ChangeTransactionTariff::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ChangeTransactionTariffRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ChangeTransactionTariffRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ChangeTransactionTariffResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ChangeTransactionTariffResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ChangeTransactionTariff::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ChangeTransactionTariff::response().validate().is_ok());
    }
}
