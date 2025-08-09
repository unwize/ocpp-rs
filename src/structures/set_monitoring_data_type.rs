use serde::{Deserialize, Serialize};
use crate::enums::monitor_enum_type::MonitorEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::periodic_event_stream_params_type::PeriodicEventStreamParamsType;
use crate::traits::OcppEntity;

/// [Context for description not available in source image]
/// This struct is used by: SetVariableMonitoringRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    /// Optional. An id SHALL only be given to replace an existing monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Optional. Monitor only active when a transaction is ongoing on a component relevant to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
    /// Required. Value for threshold or delta monitoring. For a Periodic or PeriodicClockAligned, this is the interval in seconds.
    pub value: f64,
    /// Required. The type of this monitor, e.g. a threshold or periodic monitor.
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,
    /// Required. The severity that will be assigned to an event that is triggered by this monitor.
    pub severity: i32,
    /// Required. Component for which monitor is set.
    pub component: ComponentType,
    /// Required. Variable for which monitor is set.
    pub variable: VariableType,
    /// Optional. When present, events from this monitor will be sent via a periodic event stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_event_stream: Option<PeriodicEventStreamParamsType>,
}

impl OcppEntity for SetMonitoringDataType {
    /// Validates the fields of SetMonitoringDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(id) = self.id {
            e.check_bounds("id", 0, i32::MAX, id);
        }
        e.check_bounds("severity", 0, 9, self.severity);
        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);
        if let Some(params) = &self.periodic_event_stream {
            e.check_member("periodic_event_stream", params);
        }

        e.build("SetMonitoringDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let monitoring_data = Default::default();
        assert!(monitoring_data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let monitoring_data = SetMonitoringDataType {
            id: None,
            transaction: None,
            value: 0.5,
            monitor_type: MonitorEnumType::LowerThreshold,
            severity: 9,
            component: Default::default(),
            variable: Default::default(),
            periodic_event_stream: None,
        };
        assert!(monitoring_data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_negative() {
        let mut monitoring_data = Default::default();
        monitoring_data.id = Some(-1);
        let result = monitoring_data.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "id");
            }
        }
    }

    #[test]
    fn test_validate_failure_severity_out_of_bounds() {
        let mut monitoring_data = Default::default();
        monitoring_data.severity = 10;
        let result = monitoring_data.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "severity");
            }
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = Default::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SetMonitoringDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}