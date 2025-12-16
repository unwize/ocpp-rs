use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

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

impl Default for AdditionalInfoType {
    fn default() -> Self {
        Self {
            additional_id_token: "".to_string(),
            r#type: "".to_string(),
        }
    }
}
#[typetag::serde]
impl OcppEntity for AdditionalInfoType {
    /// Validates the fields of AdditionalInfoType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality(
            "additional_id_token",
            0,
            255,
            &self.additional_id_token.chars(),
        );
        e.check_cardinality("type", 0, 50, &self.r#type.chars());

        e.build("AdditionalInfoType")
    }
}
