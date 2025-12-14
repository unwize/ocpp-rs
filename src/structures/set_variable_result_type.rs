use crate::enums::attribute_enum_type::AttributeEnumType;
use crate::enums::set_variable_status_enum_type::SetVariableStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::status_info_type::StatusInfoType;
use crate::structures::variable_type::VariableType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Used by: SetVariablesResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    /// Optional. The type of attribute: Actual, Target, MinSet, MaxSet. Default is Actual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Required. Result status of setting the variable.
    pub attribute_status: SetVariableStatusEnumType,
    /// Required. The component for which the result is returned.
    pub component: ComponentType,
    /// Required. The variable for which the result is returned.
    pub variable: VariableType,
    /// Optional. Detailed attribute status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

impl Default for SetVariableResultType {
    fn default() -> SetVariableResultType {
        Self {
            attribute_type: Some(AttributeEnumType::Actual),
            attribute_status: SetVariableStatusEnumType::Accepted,
            component: Default::default(),
            variable: Default::default(),
            attribute_status_info: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for SetVariableResultType {
    /// Validates the fields of SetVariableResultType.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);
        if let Some(status_info) = &self.attribute_status_info {
            e.check_member("attribute_status_info", status_info);
        }

        e.build("SetVariableResultType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = SetVariableResultType::default();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SetVariableResultType::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SetVariableResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
