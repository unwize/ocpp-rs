use crate::enums::authorization_status_enum_type::AuthorizationStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::id_token_type::IdTokenType;
use crate::structures::message_content_type::MessageContentType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: Implement IdTokenType
// TODO: Implement MessageContentType

/// Contains status information about an identifier.
/// Used by: Common::AuthorizationData, AuthorizeResponse, TransactionEventResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    /// Required. Current status of the ID Token.
    pub status: AuthorizationStatusEnumType,
    /// Optional. Date and Time after which the token must be considered invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,
    /// Optional. Priority from a business point of view. Default priority is 0, the range is from -9 to 9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    /// Optional. Preferred user interface language of identifier user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    /// Optional. Second preferred user interface language of identifier user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    /// Optional. Only used when the IdToken is only valid for one or more specific EVSEs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i32>>,
    /// Optional. This contains the group identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    /// Optional. Personal message that can be shown to the EV Driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
}

impl OcppEntity for IdTokenInfoType {
    /// Validates the fields of IdTokenInfoType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(priority) = self.charging_priority {
            e.check_bounds("charging_priority", -9, 9, priority);
        }

        if let Some(lang1) = &self.language1 {
            e.check_cardinality("language1", 0, 8, &lang1.chars());
        }

        if let Some(lang2) = &self.language2 {
            e.check_cardinality("language2", 0, 8, &lang2.chars());
        }

        if let Some(evse_id) = &self.evse_id {
            e.check_cardinality("evse_id", 0, 8, &evse_id.iter());

            // Manually check bounds for each ID as we cannot directly call `validate` on i32
            for i in 0..evse_id.len() {
                e.check_bounds(format!("evse_id[{i}]").as_str(), 0, i32::MAX, evse_id[i]);
            }
        }

        if let Some(id_token) = &self.group_id_token {
            e.check_member("group_id_token", id_token);
        }

        if let Some(message) = &self.personal_message {
            e.check_member("personal_message", message);
        }

        e.build("IdTokenInfoType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;
    #[test]
    fn test_validate_success() {
        let full_struct = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            charging_priority: Some(5),
            language1: Some("en-US".to_string()),
            language2: Some("fr-CA".to_string()),
            evse_id: Some(vec![1, 3]),
            group_id_token: Some(IdTokenType::default()),
            personal_message: Some(MessageContentType::default()),
        };
        assert!(full_struct.validate().is_ok());

        let minimal_struct = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            evse_id: None,
            group_id_token: None,
            personal_message: None,
        };
        assert!(minimal_struct.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_charging_priority_low() {
        let id_token_info_type = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            charging_priority: Some(-10), // Invalid
            cache_expiry_date_time: None,
            language1: None,
            language2: None,
            evse_id: None,
            group_id_token: None,
            personal_message: None,
        };
        let result = id_token_info_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "charging_priority");
            } else {
                panic!("Expected FieldValidationError for 'charging_priority'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_language1_length() {
        let id_token_info_type = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            language1: Some("a".repeat(9)), // Invalid length
            cache_expiry_date_time: None,
            charging_priority: None,
            language2: None,
            evse_id: None,
            group_id_token: None,
            personal_message: None,
        };
        let result = id_token_info_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "language1");
            } else {
                panic!("Expected FieldValidationError for 'language1'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_evse_id_bound() {
        let id_token_info_type = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            evse_id: Some(vec![1, -1, 3]), // Invalid bound
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
        };
        let result = id_token_info_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "evse_id[1]");
            } else {
                panic!("Expected FieldValidationError for 'evse_id[1]'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            charging_priority: Some(5),
            language1: Some("en-US".to_string()),
            language2: Some("fr-CA".to_string()),
            evse_id: Some(vec![1, 3]),
            group_id_token: Some(IdTokenType::default()),
            personal_message: Some(MessageContentType::default()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: IdTokenInfoType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);

        let original_struct_minimal = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            evse_id: None,
            group_id_token: None,
            personal_message: None,
        };
        let serialized_minimal = serde_json::to_string(&original_struct_minimal).unwrap();
        let deserialized_minimal: IdTokenInfoType =
            serde_json::from_str(&serialized_minimal).unwrap();
        assert_eq!(original_struct_minimal, deserialized_minimal);
    }
}
