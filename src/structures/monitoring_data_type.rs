use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::traits::OcppEntity;

/// Class to hold parameters of SetVariableMonitoring request.
/// Used by: NotifyMonitoringReportRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// Required. Component for which monitoring report was requested.
    pub component: ComponentType,
    /// Required. Variable for which monitoring report was requested.
    pub variable: VariableType,
    /// Required. List of monitors for this Component-Variable pair.
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

impl OcppEntity for MonitoringDataType {
    /// Validates the fields of MonitoringDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_member("component", &self.component);


        e.check_member("variable", &self.variable);

        e.check_cardinality("variable_monitoring", 1, usize::MAX, &self.variable_monitoring.iter());
        e.check_iter_member("variable_monitoring", self.variable_monitoring.iter());

        e.build("MonitoringDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::structures::component_type::ComponentType;

    #[test]
    fn test_validate_success() {
        let monitoring_data = MonitoringDataType {
            component: ComponentType::default(),
            variable: VariableType::default(),
            variable_monitoring: vec![VariableMonitoringType::default(); 2],
        };
        assert!(monitoring_data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let monitoring_data = MonitoringDataType {
            component: ComponentType,
            variable: VariableType,
            variable_monitoring: vec![VariableMonitoringType::default()],
        };
        assert!(monitoring_data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_empty_monitoring_list() {
        let monitoring_data = MonitoringDataType {
            component: ComponentType::default(),
            variable: VariableType::default(),
            variable_monitoring: vec![], // Invalid: empty vector
        };
        let result = monitoring_data.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "variable_monitoring");
            } else {
                panic!("Expected FieldValidationError for 'variable_monitoring'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = MonitoringDataType {
            component: ComponentType::default(),
            variable: VariableType::default(),
            variable_monitoring: vec![VariableMonitoringType::default(); 2],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: MonitoringDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}