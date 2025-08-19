use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::fixed_var_type::FixedVarType;
use crate::traits::OcppEntity;

/// Used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FixedVarGetType {
    /// Required. Id of setting.
    pub id: String,
    /// Required. True if setting is a default control.
    pub is_default: bool,
    /// Required. True if this setting is superseded by a lower priority setting.
    pub is_superseded: bool,
    /// Required. Fixed Var setpoint
    pub fixed_var: FixedVarType, 
}

impl OcppEntity for FixedVarGetType {
    /// Validates the fields of FixedVarGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id", 0, 36, &self.id.chars());

        e.check_member("fixed_var", &self.fixed_var);

        e.build("FixedVarGetType")
    }
}
