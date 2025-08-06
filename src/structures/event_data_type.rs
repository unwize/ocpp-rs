use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::enums::event_notification_enum_type::EventNotificationEnumType;
use crate::enums::event_trigger_enum_type::EventTriggerEnumType;
use crate::errors::OcppError;
use crate::structures::component_type::ComponentType;

/// Class to report an event notification for a component-variable.
/// Used by: NotifyEventRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataType {
    /// Required. Identifies the event. This field can be referred to as a cause by other events.
    /// Constraints: 0 <= val
    pub event_id: i32,
    /// Required. Timestamp of the moment the report was generated.
    pub timestamp: DateTime<Utc>,
    /// Required. Type of trigger for this event, e.g. exceeding a threshold value.
    pub trigger: EventTriggerEnumType,
    /// Optional. Refers to the Id of an event that is considered to be the cause for this event.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i32>,
    /// Required. Actual value (attributeType Actual) of the variable.
    /// String length: 0..2500
    pub actual_value: String,
    /// Optional. Technical (error) code as reported by component.
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_code: Option<String>,
    /// Optional. Technical detail information as reported by component.
    /// String length: 0..500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
    /// Optional. Cleared is set to true to report the clearing of a monitored situation, i.e. a 'return to normal'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    /// Optional. If an event notification is linked to a specific transaction, this field can be used to specify its transactionId.
    /// String length: 0..36
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Optional. Identifies the VariableMonitoring which triggered the event.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_monitoring_id: Option<i32>,
    /// Optional. Specifies the event notification type of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_notification_type: Option<EventNotificationEnumType>,
    /// Optional. Severity associated with the monitor in variableMonitoringId or with the hardwired notification.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    /// Required. Component for which event is notified.
    pub component: ComponentType,
    /// Required. Variable for which event is notified.
    pub variable: VariableType, // TODO: Implement VariableType
}

impl EventDataType {
    /// Validates the fields of EventDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate event_id
        if self.event_id < 0 {
            errors.push(OcppError::FieldValidationError {
                field: "event_id".to_string(),
                related: vec![OcppError::FieldBoundsError {
                    value: self.event_id.to_string(),
                    lower: "0".to_string(),
                    upper: "MAX_INT".to_string(),
                }],
            });
        }

        // Validate cause
        if let Some(c) = self.cause {
            if c < 0 {
                errors.push(OcppError::FieldValidationError {
                    field: "cause".to_string(),
                    related: vec![OcppError::FieldBoundsError {
                        value: c.to_string(),
                        lower: "0".to_string(),
                        upper: "MAX_INT".to_string(),
                    }],
                });
            }
        }

        // Validate actual_value length
        if self.actual_value.len() > 2500 {
            errors.push(OcppError::FieldValidationError {
                field: "actual_value".to_string(),
                related: vec![OcppError::FieldCardinalityError {
                    cardinality: self.actual_value.len(),
                    lower: 0,
                    upper: 2500,
                }],
            });
        }

        // Validate tech_code length
        if let Some(tc) = &self.tech_code {
            if tc.len() > 50 {
                errors.push(OcppError::FieldValidationError {
                    field: "tech_code".to_string(),
                    related: vec![OcppError::FieldCardinalityError {
                        cardinality: tc.len(),
                        lower: 0,
                        upper: 50,
                    }],
                });
            }
        }

        // Validate tech_info length
        if let Some(ti) = &self.tech_info {
            if ti.len() > 500 {
                errors.push(OcppError::FieldValidationError {
                    field: "tech_info".to_string(),
                    related: vec![OcppError::FieldCardinalityError {
                        cardinality: ti.len(),
                        lower: 0,
                        upper: 500,
                    }],
                });
            }
        }

        // Validate transaction_id length
        if let Some(tid) = &self.transaction_id {
            if tid.len() > 36 {
                errors.push(OcppError::FieldValidationError {
                    field: "transaction_id".to_string(),
                    related: vec![OcppError::FieldCardinalityError {
                        cardinality: tid.len(),
                        lower: 0,
                        upper: 36,
                    }],
                });
            }
        }

        // Validate variable_monitoring_id
        if let Some(vmid) = self.variable_monitoring_id {
            if vmid < 0 {
                errors.push(OcppError::FieldValidationError {
                    field: "variable_monitoring_id".to_string(),
                    related: vec![OcppError::FieldBoundsError {
                        value: vmid.to_string(),
                        lower: "0".to_string(),
                        upper: "MAX_INT".to_string(),
                    }],
                });
            }
        }

        // Validate severity
        if let Some(s) = self.severity {
            if s < 0 {
                errors.push(OcppError::FieldValidationError {
                    field: "severity".to_string(),
                    related: vec![OcppError::FieldBoundsError {
                        value: s.to_string(),
                        lower: "0".to_string(),
                        upper: "MAX_INT".to_string(),
                    }],
                });
            }
        }

        // No validation for 'trigger', 'event_notification_type', 'component', 'variable' without their type definitions.

        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EventDataType".to_string(),
                related: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_serialization_deserialization() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic, // Placeholder
            cause: Some(0),
            actual_value: "245.5".to_string(),
            tech_code: Some("VOLT_HIGH".to_string()),
            tech_info: Some("Voltage reading above threshold for 5s".to_string()),
            cleared: Some(false),
            transaction_id: Some("TXN12345".to_string()),
            variable_monitoring_id: Some(101),
            event_notification_type: Some(EventNotificationEnumType::HardWiredMonitor), // Placeholder
            severity: Some(5),
            component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            },
            variable: "VoltageL1".to_string(), // Placeholder
        };

        let serialized = serde_json::to_string(&event_data).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EventDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event_data, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let event_data_minimal = EventDataType {
            event_id: 0,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic,
            cause: None,
            actual_value: "10.0".to_string(),
            tech_code: None,
            tech_info: None,
            cleared: None,
            transaction_id: None,
            variable_monitoring_id: None,
            event_notification_type: None,
            severity: None,
            component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            },
            variable: "Current".to_string(),
        };
        assert!(event_data_minimal.validate().is_ok());

        let event_data_full_lengths = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Alerting,
            cause: Some(10),
            actual_value: "a".repeat(2500), // Max length
            tech_code: Some("b".repeat(50)), // Max length
            tech_info: Some("c".repeat(500)), // Max length
            cleared: Some(true),
            transaction_id: Some("d".repeat(36)), // Max length
            variable_monitoring_id: Some(999),
            event_notification_type: Some(EventNotificationEnumType::PreconfiguredMonitor),
            severity: Some(10),
            component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            },
            variable: "VariableB".to_string(),
        };
        assert!(event_data_full_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_event_id() {
        let event_data = EventDataType {
            event_id: -1, // Invalid
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Alerting,
            cause: None, actual_value: "val".to_string(), tech_code: None, tech_info: None,
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["event_id".to_string()]);
    }

    #[test]
    fn test_validation_invalid_cause() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(1998, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Delta,
            cause: Some(-5), // Invalid
            actual_value: "val".to_string(), tech_code: None, tech_info: None,
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["cause".to_string()]);
    }

    #[test]
    fn test_validation_actual_value_too_long() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1,10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic,
            cause: None,
            actual_value: "a".repeat(2501), // Invalid
            tech_code: None, tech_info: None,
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["actual_value".to_string()]);
    }

    #[test]
    fn test_validation_tech_code_too_long() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic,
            cause: None, actual_value: "val".to_string(),
            tech_code: Some("a".repeat(51)), // Invalid
            tech_info: None,
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["tech_code".to_string()]);
    }

    #[test]
    fn test_validation_tech_info_too_long() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Delta,
            cause: None, actual_value: "val".to_string(), tech_code: None,
            tech_info: Some("a".repeat(501)), // Invalid
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["tech_info".to_string()]);
    }

    #[test]
    fn test_validation_transaction_id_too_long() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Alerting,
            cause: None, actual_value: "val".to_string(), tech_code: None, tech_info: None,
            cleared: None,
            transaction_id: Some("a".repeat(37)), // Invalid
            variable_monitoring_id: None,
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["transaction_id".to_string()]);
    }

    #[test]
    fn test_validation_invalid_variable_monitoring_id() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic,
            cause: None, actual_value: "val".to_string(), tech_code: None, tech_info: None,
            cleared: None, transaction_id: None,
            variable_monitoring_id: Some(-10), // Invalid
            event_notification_type: None, severity: None, component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["variable_monitoring_id".to_string()]);
    }

    #[test]
    fn test_validation_invalid_severity() {
        let event_data = EventDataType {
            event_id: 1,
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Periodic,
            cause: None, actual_value: "val".to_string(), tech_code: None, tech_info: None,
            cleared: None, transaction_id: None, variable_monitoring_id: None,
            event_notification_type: None,
            severity: Some(-1), // Invalid
            component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec!["severity".to_string()]);
    }

    #[test]
    fn test_validation_multiple_errors() {
        let event_data = EventDataType {
            event_id: -1, // Invalid 1
            timestamp: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            trigger: EventTriggerEnumType::Alerting,
            cause: Some(-5), // Invalid 2
            actual_value: "a".repeat(2501), // Invalid 3
            tech_code: Some("b".repeat(51)), // Invalid 4
            tech_info: Some("c".repeat(501)), // Invalid 5
            cleared: Some(false),
            transaction_id: Some("d".repeat(37)), // Invalid 6
            variable_monitoring_id: Some(-10), // Invalid 7
            event_notification_type: None,
            severity: Some(-1), // Invalid 8
            component: ComponentType {
                name: "EVSE".to_string(),
                instance: None,
                evse: None,
            }, variable: "Var".to_string(),
        };
        let err = event_data.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "event_id".to_string(),
            "cause".to_string(),
            "actual_value".to_string(),
            "tech_code".to_string(),
            "tech_info".to_string(),
            "transaction_id".to_string(),
            "variable_monitoring_id".to_string(),
            "severity".to_string(),
        ]);
    }
}