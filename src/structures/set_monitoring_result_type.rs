use serde::{Deserialize, Serialize};
use crate::enums::monitor_enum_type::MonitorEnumType;
use crate::enums::set_monitoring_status_enum_type::SetMonitoringStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::OcppEntity;

/// Class to hold result of SetVariableMonitoring request.
/// Used by: SetVariableMonitoringResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    /// Optional. Id given to the VariableMonitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Required. Indicates if the monitor could be returned.
    pub status: SetMonitoringStatusEnumType,
    /// Required. The type of this monitor.
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,
    /// Required. The severity level (0-9) assigned to the monitor.
    pub severity: i32,
    /// Required. The component for which the status is returned.
    pub component: ComponentType,
    /// Required. The variable for which the status is returned.
    pub variable: VariableType,
    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for SetMonitoringResultType {
    /// Validates the fields of SetMonitoringResultType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(id) = self.id {
            e.check_bounds("id", 0, i32::MAX, id);
        }
        e.check_bounds("severity", 0, 9, self.severity);
        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);
        if let Some(status_info) = &self.status_info {
            e.check_member("status_info", status_info);
        }

        e.build("SetMonitoringResultType")
    }
}

impl Default for SetMonitoringResultType {
    fn default() -> SetMonitoringResultType {
        Self {
            id: None,
            status: SetMonitoringStatusEnumType::default(),
            monitor_type: MonitorEnumType::Periodic,
            severity: 0,
            component: Default::default(),
            variable: Default::default(),
            status_info: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let result = SetMonitoringResultType {
            id: Some(123),
            severity: 5,
            ..Default::default()
        };
        assert!(result.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let result = SetMonitoringResultType::default();
        assert!(result.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_severity_out_of_bounds() {
        let mut result = SetMonitoringResultType::default();
        result.severity = 10;
        assert!(result.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SetMonitoringResultType::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SetMonitoringResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}