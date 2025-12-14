use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::variable_type::VariableType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}
#[typetag::serde]
impl OcppEntity for ComponentVariableType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_member("component", &self.component);
        if let Some(variable) = &self.variable {
            e.check_member("variable", variable);
        }

        e.build("ComponentVariableType")
    }
}
