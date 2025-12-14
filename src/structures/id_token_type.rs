use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::additional_info_type::AdditionalInfoType;
use crate::traits::OcppEntity;

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
/// Used by: Common::AuthorizationData, Common::IdTokenInfoType, RequestStartTransactionRequest, AuthorizeRequest, TransactionEventResponse, ReserveNowRequest, CustomerInformationRequest, BatterySwapRequest, RequestBatterySwapRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// Required. IdToken is case insensitive. Might hold the hidden id of an RFID tag, but can for example also contain a UUID.
    pub id_token: String,
    /// Required. Enumeration of possible idToken types.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional. AdditionalInfo can be used to send extra information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
}

impl Default for IdTokenType {
    fn default() -> Self {
        Self {
            id_token: "".to_string(),
            r#type: "".to_string(),
            additional_info: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for IdTokenType {
    /// Validates the fields of IdTokenType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id_token", 0, 255, &self.id_token.chars());
        e.check_cardinality("type", 0, 20, &self.r#type.chars());

        if let Some(additional_info) = &self.additional_info {
            e.check_iter_member("additional_info", additional_info.iter());
        }

        e.build("IdTokenType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::additional_info_type::AdditionalInfoType;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let id_token_type = IdTokenType {
            id_token: "my_token_123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: Some(vec![AdditionalInfoType::default()]),
        };
        assert!(id_token_type.validate().is_ok());

        let id_token_type_minimal = IdTokenType {
            id_token: "my_token_123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: None,
        };
        assert!(id_token_type_minimal.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_token_length() {
        let id_token_type = IdTokenType {
            id_token: "a".repeat(256), // Invalid length
            r#type: "RFID".to_string(),
            additional_info: None,
        };
        let result = id_token_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "id_token");
            } else {
                panic!("Expected FieldValidationError for 'id_token'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_type_length() {
        let id_token_type = IdTokenType {
            id_token: "my_token_123".to_string(),
            r#type: "a".repeat(21), // Invalid length
            additional_info: None,
        };
        let result = id_token_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "type");
            } else {
                panic!("Expected FieldValidationError for 'type'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = IdTokenType {
            id_token: "my_token_123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: Some(vec![AdditionalInfoType::default()]),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: IdTokenType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
