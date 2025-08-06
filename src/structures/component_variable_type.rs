use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>, // TODO: Implement VariableType
}

impl OcppEntity for ComponentVariableType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.push_member("component", &self.component);
        if let Some(variable) = &self.variable {
            e.push_member("variable", variable);
        }
        
        e.build("ComponentVariableType")
    }
}