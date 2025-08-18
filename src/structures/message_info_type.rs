use crate::enums::message_priority_enum_type::MessagePriorityEnumType;
use crate::enums::message_state_enum_type::MessageStateEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::message_content_type::MessageContentType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contains message details for a message to be displayed on a Charging Station.
/// Used by: SetDisplayMessageRequest, NotifyDisplayMessagesRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Required. Unique id within an exchange context. It is defined within the OCPP context as a positive integer value (greater or equal to zero).
    pub id: i32,
    /// Required. With what priority should this message be shown.
    pub priority: MessagePriorityEnumType, // TODO: Implement MessagePriorityEnumType
    /// Optional. During what state should this message be shown. When omitted this message should be shown in any state of the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>, // TODO: Implement MessageStateEnumType
    /// Optional. From what date-time should this message be shown. If omitted: directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<DateTime<Utc>>,
    /// Optional. Until what date-time should this message be shown, after this date/time this message SHALL be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,
    /// Optional. During which transaction shall this message be shown. Message SHALL be removed by the Charging Station after transaction has ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Required. Contains message details for the message to be displayed on a Charging Station.
    pub message: MessageContentType,
    /// Optional. When a Charging Station has multiple Displays, this field can be used to define to which Display this message belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,
    /// Optional. Contains message details for extra languages to be displayed on a Charging Station.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub message_extra: Vec<MessageContentType>,
}

impl OcppEntity for MessageInfoType {
    /// Validates the fields of MessageInfoType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("id", 0, i32::MAX, self.id);

        if let (Some(start), Some(end)) = (&self.start_date_time, &self.end_date_time) {
            if start > end {
                e.push_relation_error(
                    "start_date_time",
                    "end_date_time",
                    "must be before or equal to end_date_time",
                );
            }
        }

        if let Some(id) = &self.transaction_id {
            e.check_cardinality("transaction_id", 0, 36, &id.chars());
        }

        e.check_member("message", &self.message);

        if let Some(display_component) = &self.display {
            e.check_member("display", display_component);
        }

        e.check_cardinality("message_extra", 0, 4, &self.message_extra.iter());
        e.check_iter_member("message_extra", self.message_extra.iter());

        e.build("MessageInfoType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let message_info = MessageInfoType {
            id: 1,
            priority: MessagePriorityEnumType::AlwaysFront,
            state: Some(MessageStateEnumType::Charging),
            start_date_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            end_date_time: Some(Utc.timestamp_opt(1672534800, 0).unwrap()),
            transaction_id: Some("tx-12345".to_string()),
            message: MessageContentType::default(),
            display: Some(ComponentType::default()),
            message_extra: vec![MessageContentType::default(), MessageContentType::default()],
        };
        assert!(message_info.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let message_info = MessageInfoType {
            id: 1,
            priority: MessagePriorityEnumType::AlwaysFront,
            state: None,
            start_date_time: None,
            end_date_time: None,
            transaction_id: None,
            message: MessageContentType::default(),
            display: None,
            message_extra: vec![],
        };
        assert!(message_info.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_low() {
        let message_info = MessageInfoType {
            id: -1, // Invalid
            priority: MessagePriorityEnumType::AlwaysFront,
            state: None,
            start_date_time: None,
            end_date_time: None,
            transaction_id: None,
            message: MessageContentType::default(),
            display: None,
            message_extra: vec![],
        };
        let err = message_info.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["id"]);
    }

    #[test]
    fn test_validate_failure_date_range() {
        let message_info = MessageInfoType {
            id: 1,
            priority: MessagePriorityEnumType::AlwaysFront,
            state: None,
            start_date_time: Some(Utc.timestamp_opt(1672534800, 0).unwrap()),
            end_date_time: Some(Utc.timestamp_opt(167253200, 0).unwrap()), // Invalid
            transaction_id: None,
            message: MessageContentType::default(),
            display: None,
            message_extra: vec![],
        };
        let err = message_info.validate().unwrap_err();
        assert_num_field_errors(&err, 2);
        assert_invalid_fields(&err, &["end_date_time", "start_date_time"]);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = MessageInfoType {
            id: 1,
            priority: MessagePriorityEnumType::AlwaysFront,
            state: Some(MessageStateEnumType::Charging),
            start_date_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            end_date_time: Some(Utc.timestamp_opt(1672534800, 0).unwrap()),
            transaction_id: Some("tx-12345".to_string()),
            message: MessageContentType::default(),
            display: Some(ComponentType::default()),
            message_extra: vec![Default::default(); 2],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: MessageInfoType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
