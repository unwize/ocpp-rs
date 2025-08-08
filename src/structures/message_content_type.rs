use serde::{Deserialize, Serialize};
use crate::enums::message_format_enum_type::MessageFormatEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Contains message details, for a message to be displayed on a Charging Station.
/// Used by: Common::IdTokenInfoType, Common::MessageInfoType, Common::TariffType, TransactionEventResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    /// Required. Format of the message.
    pub format: MessageFormatEnumType,
    /// Optional. Message language identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Required. Message contents.
    pub content: String,
}

impl Default for MessageContentType {
    fn default() -> Self {
        Self {
            format: MessageFormatEnumType::ASCII,
            language: None,
            content: "".to_string(),
        }
    }
}

impl OcppEntity for MessageContentType {
    /// Validates the fields of MessageContentType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(lang) = &self.language {
            e.check_cardinality("language", 0, 8, &lang.chars());
        }

        e.check_cardinality("content", 0, 1024, &self.content.chars());

        e.build("MessageContentType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::enums::message_format_enum_type::MessageFormatEnumType;

    #[test]
    fn test_validate_success_full() {
        let message_content_type = MessageContentType {
            format: MessageFormatEnumType::ASCII,
            language: Some("en-US".to_string()),
            content: "Hello World!".to_string(),
        };
        assert!(message_content_type.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let message_content_type = MessageContentType {
            format: MessageFormatEnumType::ASCII,
            language: None,
            content: "Hello World!".to_string(),
        };
        assert!(message_content_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_language_length() {
        let message_content_type = MessageContentType {
            format: MessageFormatEnumType::ASCII,
            language: Some("a".repeat(9)), // Invalid length
            content: "Hello World!".to_string(),
        };
        let result = message_content_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "language");
            } else {
                panic!("Expected FieldValidationError for 'language'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_content_length() {
        let message_content_type = MessageContentType {
            format: MessageFormatEnumType::ASCII,
            language: None,
            content: "a".repeat(1025), // Invalid length
        };
        let result = message_content_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "content");
            } else {
                panic!("Expected FieldValidationError for 'content'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = MessageContentType {
            format: MessageFormatEnumType::ASCII,
            language: Some("en-US".to_string()),
            content: "Hello World!".to_string(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: MessageContentType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}