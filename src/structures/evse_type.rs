use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Electric Vehicle Supply Equipment
/// Used by: Common::ComponentType, TriggerMessageRequest, ChangeAvailabilityRequest, TransactionEventRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EVSEType {
    /// Required. EVSE Identifier. This contains a number (> 0) designating an EVSE of the Charging Station.
    pub id: i32,
    /// Optional. An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

impl Default for EVSEType {
    fn default() -> Self {
        Self {
            id: 1,
            connector_id: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for EVSEType {
    /// Validates the fields of EVSEType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("id", 1, i32::MAX, self.id);

        if let Some(connector_id) = self.connector_id {
            e.check_bounds("connector_id", 0, i32::MAX, connector_id);
        }

        e.build("EVSEType")
    }
}
