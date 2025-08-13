use crate::errors::OcppError;
use crate::structures::id_token_info_type::IdTokenInfoType;
use crate::structures::id_token_type::IdTokenType;
use crate::traits::OcppEntity;
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

impl OcppEntity for AuthorizationData {
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        if let Err(e) = self.id_token_info.validate() {
            errors.push(e.to_field_validation_error("id_token_info"));
        }

        if let Err(e) = self.id_token.validate() {
            errors.push(e.to_field_validation_error("id_token"));
        }

        Ok(())
    }
}
