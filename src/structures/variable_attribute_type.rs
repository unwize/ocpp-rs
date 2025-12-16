use crate::enums::attribute_enum_type::AttributeEnumType;
use crate::enums::mutability_enum_type::MutabilityEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    /// Optional. Type of attribute (e.g., Actual, MinSet, MaxSet). Defaults to Actual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Optional. Value of the attribute. May be omitted if mutability is 'WriteOnly'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Optional. Defines the mutability of the attribute. Defaults to ReadWrite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<MutabilityEnumType>,
    /// Optional. If true, the value is persistent across reboots. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    /// Optional. If true, the value cannot be changed at runtime. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}

impl Default for VariableAttributeType {
    fn default() -> Self {
        Self {
            attribute_type: Some(AttributeEnumType::Actual),
            value: Some(String::new()),
            mutability: Some(MutabilityEnumType::ReadWrite),
            persistent: Some(false),
            constant: Some(false),
        }
    }
}
#[typetag::serde]
impl OcppEntity for VariableAttributeType {
    /// Validates the fields of VariableAttributeType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(value) = &self.value {
            e.check_cardinality("value", 0, 2500, &value.chars());
        }

        if self.attribute_type.is_none()
            && let Some(mutability) = &self.mutability
            && *mutability != MutabilityEnumType::WriteOnly
        {
            e.push_relation_error(
                "value",
                "mutability",
                "`value` may only be `None` when `mutability` is set to `WriteOnly`",
            )
        }

        e.build("VariableAttributeType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> VariableAttributeType {
        VariableAttributeType {
            attribute_type: Some(AttributeEnumType::Actual),
            value: Some("test_value".to_string()),
            mutability: Some(MutabilityEnumType::ReadWrite),
            persistent: Some(true),
            constant: Some(false),
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_fields_present() {
        let data = VariableAttributeType {
            attribute_type: None,
            value: None,
            mutability: None,
            persistent: None,
            constant: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_value_too_long() {
        let mut data = create_test_instance();
        data.value = Some("a".repeat(2501));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VariableAttributeType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
