use crate::enums::data_transfer_status_enum_type::DataTransferStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.20. DataTransfer
pub struct DataTransfer;

impl OcppMessage for DataTransfer {
    type Request = DataTransferRequest;
    type Response = DataTransferResponse;
}

/// 1.20.1. DataTransferRequest
/// This contains the field definition of the DataTransferRequest PDU sent either by the CSMS to the Charging Station or vice versa.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// Optional. May be used to indicate a specific message or implementation.
    pub message_id: Option<String>,
    /// Optional. Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    /// Note: 'anyType' is often represented as a String or a flexible type like serde_json::Value
    pub data: Option<String>,
    /// Required. This identifies the Vendor specific implementation.
    pub vendor_id: String,
}
#[typetag::serde]
impl OcppEntity for DataTransferRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(message_id) = &self.message_id {
            b.check_cardinality("message_id", 0, 50, &message_id.chars());
        }

        // 'data' is anyType, no strict validation is applied to its content or size as per OCPP spec.

        b.check_cardinality("vendor_id", 0, 255, &self.vendor_id.chars());

        b.build("DataTransferRequest")
    }
}

#[typetag::serde]
impl OcppRequest for DataTransferRequest {
    fn get_message_type(&self) -> String {
        String::from("DataTransfer")
    }
}

/// 1.20.2. DataTransferResponse
/// This contains the field definition of the DataTransferResponse PDU sent by the Charging Station to the CSMS or vice versa in response to a DataTransferRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatusEnumType,
    /// Optional. Data without specified length or format, in response to request.
    /// Note: 'anyType' is often represented as a String or a flexible type like serde_json::Value
    pub data: Option<String>,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for DataTransferResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("DataTransferResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = DataTransfer::request();
        let resp = DataTransfer::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = DataTransferRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: DataTransferRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = DataTransferResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: DataTransferResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(DataTransfer::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(DataTransfer::response().validate().is_ok());
    }
}
