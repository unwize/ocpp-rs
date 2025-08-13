use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::fixed_pf_type::FixedPFType;
use crate::traits::OcppEntity;

/// Used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FixedPFGetType {
    /// Required. Id of setting.
    pub id: String,
    /// Required. True if setting is a default control.
    pub is_default: bool,
    /// Required. True if this setting is superseded by a lower priority setting.
    pub is_superseded: bool,
    /// Required. FixedPF for AbsorbW or InjectW
    pub fixed_pf: FixedPFType, // TODO: Implement FixedPFType
}

impl OcppEntity for FixedPFGetType {
    /// Validates the fields of FixedPFGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id", 0, 36, &self.id.chars());

        e.check_member("fixed_pf", &self.fixed_pf);

        e.build("FixedPFGetType")
    }
}
