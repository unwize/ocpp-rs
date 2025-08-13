use crate::enums::data_enum_type::DataEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    /// Optional. Unit of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Required. Data type of this variable.
    pub data_type: DataEnumType,
    /// Optional. Minimum possible value of this variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,
    /// Optional. Maximum possible value of this variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,
    /// Optional. Maximum number of elements from valuesList that are supported as attributeValue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_elements: Option<i32>,
    /// Optional. Specifies the allowed values for the type when dataType is OptionList, MemberList or SequenceList.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    /// Required. Flag indicating if this variable supports monitoring.
    pub supports_monitoring: bool,
}

impl Default for VariableCharacteristicsType {
    fn default() -> Self {
        Self {
            unit: None,
            data_type: DataEnumType::String,
            min_limit: None,
            max_limit: None,
            max_elements: None,
            values_list: None,
            supports_monitoring: false,
        }
    }
}

impl OcppEntity for VariableCharacteristicsType {
    /// Validates the fields of VariableCharacteristicsType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(unit) = &self.unit {
            e.check_cardinality("unit", 0, 16, &unit.chars());
        }

        if let Some(max_elements) = self.max_elements {
            // maxElements must be >= 1
            e.check_bounds("max_elements", 1, i32::MAX, max_elements);
        }

        if let Some(values_list) = &self.values_list {
            e.check_cardinality("values_list", 0, 1000, &values_list.chars());
        }

        e.build("VariableCharacteristicsType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::data_enum_type::DataEnumType;
    use serde_json;

    fn create_test_instance() -> VariableCharacteristicsType {
        VariableCharacteristicsType {
            unit: Some("kW".to_string()),
            data_type: DataEnumType::Decimal,
            min_limit: Some(0.0),
            max_limit: Some(100.0),
            max_elements: Some(10),
            values_list: Some("1,2,3,4".to_string()),
            supports_monitoring: true,
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal_fields() {
        let data = VariableCharacteristicsType {
            unit: None,
            data_type: DataEnumType::String,
            min_limit: None,
            max_limit: None,
            max_elements: None,
            values_list: None,
            supports_monitoring: false,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_unit_too_long() {
        let mut data = create_test_instance();
        data.unit = Some("a".repeat(17));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_max_elements_too_small() {
        let mut data = create_test_instance();
        data.max_elements = Some(0);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_values_list_too_long() {
        let mut data = create_test_instance();
        data.values_list = Some("a".repeat(1001));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VariableCharacteristicsType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
