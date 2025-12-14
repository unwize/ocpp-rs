use crate::enums::event_notification_enum_type::EventNotificationEnumType;
use crate::enums::monitor_enum_type::MonitorEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    /// Required. Identifies the monitor. Must be non-negative.
    pub id: i32,
    /// Required. Indicates if the monitor is only active when a transaction is ongoing.
    pub transaction: bool,
    /// Required. Value for threshold or delta monitoring. For periodic monitors, this is the interval in seconds.
    pub value: f64,
    /// Required. The type of this monitor, e.g., a threshold, delta, or periodic monitor.
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,
    /// Required. The severity level (0-9) assigned to an event triggered by this monitor. 0 is the highest severity.
    pub severity: i32,
    /// Required. Type of monitor for event notifications.
    pub event_notification_type: EventNotificationEnumType,
}

impl Default for VariableMonitoringType {
    fn default() -> VariableMonitoringType {
        Self {
            id: 0,
            transaction: false,
            value: 0.0,
            monitor_type: MonitorEnumType::Periodic,
            severity: 9,
            event_notification_type: EventNotificationEnumType::HardWiredNotification,
        }
    }
}
#[typetag::serde]
impl OcppEntity for VariableMonitoringType {
    /// Validates the fields of VariableMonitoringType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("id", 0, i32::MAX, self.id);

        e.check_bounds("severity", 0, 9, self.severity);

        e.build("VariableMonitoringType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::event_notification_enum_type::EventNotificationEnumType;
    use crate::enums::monitor_enum_type::MonitorEnumType;
    use serde_json;

    fn create_test_instance() -> VariableMonitoringType {
        VariableMonitoringType {
            id: 1,
            transaction: true,
            value: 100.0,
            monitor_type: MonitorEnumType::UpperThreshold,
            severity: 5,
            event_notification_type: EventNotificationEnumType::PreconfiguredMonitor,
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_id() {
        let mut data = create_test_instance();
        data.id = -1;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_severity_too_high() {
        let mut data = create_test_instance();
        data.severity = 10;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: VariableMonitoringType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
