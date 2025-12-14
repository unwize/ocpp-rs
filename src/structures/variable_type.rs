use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    /// Required. Name of the variable. Max length of 50.
    pub name: String,
    /// Optional. Name of instance in case the variable has multiple instances. Max length of 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl Default for VariableType {
    fn default() -> VariableType {
        Self {
            name: "a".to_string(),
            instance: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for VariableType {
    /// Validates the fields of VariableType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("name", 1, 50, &self.name.chars());

        if let Some(instance) = &self.instance {
            e.check_cardinality("instance", 0, 50, &instance.chars());
        }

        e.build("VariableType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> VariableType {
        VariableType {
            name: "testName".to_string(),
            instance: Some("testInstance".to_string()),
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal_fields() {
        let data = VariableType {
            name: "testName".to_string(),
            instance: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_name_too_long() {
        let mut data = create_test_instance();
        data.name = "a".repeat(51);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_name_empty() {
        let mut data = create_test_instance();
        data.name = "".to_string();
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_instance_too_long() {
        let mut data = create_test_instance();
        data.instance = Some("a".repeat(51));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VariableType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
