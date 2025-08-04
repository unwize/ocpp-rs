use serde::{Deserialize, Serialize};
use crate::errors::OcppError;
use crate::errors::OcppError::{FieldCardinalityError, FieldValidationError, StructureValidationError};
use crate::traits::OcppEntity;

/// Contains a case-insensitive identifier to use for the authorization
/// and the type of authorization to support multiple forms of identifiers.
/// Used by: Common::IdTokenType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AdditionalInfoType {
    /// Required. This field specifies the additional IdToken.
    /// String length: 0..255
    pub additional_id_token: String,
    /// Required. additionalInfo can be used to send extra information to CSMS in addition to the regular
    /// authorization with IdToken. AdditionalInfo contains one or more custom types, which need to be agreed upon by all
    /// parties involved. When the type is not supported, the CSMS/Charging Station MAY ignore the additionalInfo.
    /// String length: 0..50
    pub r#type: String, // 'type' is a Rust keyword, so we use r#type to escape it
}

impl OcppEntity for AdditionalInfoType {
    /// Validates the fields of AdditionalInfoType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate additional_id_token length (0 to 255)
        if self.additional_id_token.len() > 255 {
           errors.push(
                FieldCardinalityError {
                    cardinality: self.additional_id_token.len(),
                    lower: 0,
                    upper: 255,
                }.to_field_validation_error("additional_id_token")
           )
        }

        // Validate type length (0 to 50)
        if self.r#type.len() > 50 {
           errors.push(
               FieldCardinalityError {
                   cardinality: self.r#type.len(),
                   lower: 0,
                   upper: 50,
               }.to_field_validation_error("type")
           )
        }

        if !errors.is_empty() {
            return Err(StructureValidationError {
                structure: "AdditionalInfoType".to_string(),
                source: errors,
            })
        }

        Ok(())
    }
}