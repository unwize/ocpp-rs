use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::variable_attribute_type::VariableAttributeType;
use crate::structures::variable_characteristics_type::VariableCharacteristicsType;
use crate::structures::variable_type::VariableType;
use crate::traits::OcppEntity;

/// Class to report components, variables and variable attributes and characteristics.
/// Used by: NotifyReportRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    /// Required. Component for which a report of Variable is requested.
    pub component: ComponentType,
    /// Required. Variable for which report is requested.
    pub variable: VariableType,
    /// Required. Attribute data of a variable.
    pub variable_attribute: Vec<VariableAttributeType>,
    /// Optional. Fixed read-only parameters of a variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}

impl OcppEntity for ReportDataType {
    /// Validates the fields of ReportDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);
        e.check_cardinality("variable_attribute", 1, 4, &self.variable_attribute.iter());
        for (i, attr) in self.variable_attribute.iter().enumerate() {
            e.check_member(&format!("variable_attribute[{}]", i), attr);
        }
        if let Some(characteristics) = &self.variable_characteristics {
            e.check_member("variable_characteristics", characteristics);
        }

        e.build("ReportDataType")
    }
}

impl Default for ReportDataType {
    fn default() -> Self {
        Self {
            component: Default::default(),
            variable: Default::default(),
            variable_attribute: vec![Default::default()],
            variable_characteristics: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let report = ReportDataType::default();
        assert!(report.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_variable_attribute_too_few() {
        let mut report = ReportDataType::default();
        report.variable_attribute = vec![];
        let result = report.validate();
        let err = report.validate().unwrap_err();
        assert_invalid_fields(&err, &["variable_attribute"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_variable_attribute_too_many() {
        let mut report = ReportDataType::default();
        report.variable_attribute = vec![VariableAttributeType::default(); 5]; // 5 elements, max is 4
        let result = report.validate();
        let err = report.validate().unwrap_err();
        assert_invalid_fields(&err, &["variable_attribute"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = ReportDataType::default();

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: ReportDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
