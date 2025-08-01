use serde::{Deserialize, Serialize};
use crate::errors::OcppError;
use crate::errors::OcppError::StructureValidationError;
use crate::structures::component_type::ComponentType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>, // TODO: Implement VariableType
}

impl ComponentVariableType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();
        if let Err(e) = self.component.validate() {
            errors.push(e);
        }
        if let Err(e) = self.variable.validate() {
            errors.push(e);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(StructureValidationError {
                structure: "ComponentVariableType",
                source: errors
            })
        }
    }
}