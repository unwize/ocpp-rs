use serde::{Deserialize, Serialize};

/// Contains a case-insensitive identifier to use for the authorization
/// and the type of authorization to support multiple forms of identifiers.
/// Used by: Common::IdTokenType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

impl AdditionalInfoType {
    /// Validates the fields of AdditionalInfoType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate additional_id_token length (0 to 255)
        if self.additional_id_token.len() > 255 {
            // println!("Validation failed: additional_id_token length exceeds 255.");
            return false;
        }

        // Validate type length (0 to 50)
        if self.r#type.len() > 50 {
            // println!("Validation failed: type length exceeds 50.");
            return false;
        }

        true
    }
}