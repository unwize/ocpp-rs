use serde::{Deserialize, Serialize};

/// Contains the identifier to use for authorization.
/// Used by: SendLocalListRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationData {
    /// Optional. Required when UpdateType is Full. This contains information about authorization status,
    /// expiry and group id. For a Differential update the following applies: If this element is present,
    /// then this entry SHALL be added or updated in the Local Authorization List. If this element is absent,
    /// the entry for this IdToken in the Local Authorization List SHALL be deleted.
    pub id_token_info: IdTokenInfoType, // TODO: Implement IdTokenInfoType
    /// Required. This contains the identifier which needs to be stored for authorization.
    pub id_token: IdTokenType, // TODO: Implement IdTokenType
}

impl AuthorizationData {
    /// Validates the fields of AuthorizationData.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // No specific validation rules can be applied without the definitions
        // of IdTokenInfoType and IdTokenType.
        // If IdTokenInfoType and IdTokenType were structs with their own validate methods,
        // you would call them here:
        // if let Some(info) = &self.id_token_info {
        //     if !info.validate() { return false; }
        // }
        // if !self.id_token.validate() { return false; }

        true
    }
}