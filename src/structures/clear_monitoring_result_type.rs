use crate::enums::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Result of a clear monitoring request.
/// Used by: ClearVariableMonitoringResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearMonitoringResultType {
    /// Required. Result of the clear request for this monitor, identified by its id.
    pub status: ClearMonitoringStatusEnumType,
    /// Required. Id of the monitor of which a clear was requested.
    /// Constraints: 0 <= val
    pub id: i32,
    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for ClearMonitoringResultType {
    /// Validates the fields of ClearMonitoringResultType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("id", 0, i32::MAX, self.id);

        if let Some(status_info) = &self.status_info {
            e.check_member("status_info", status_info);
        }

        e.build("ClearMonitoringResultType")
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let result = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Accepted,
            id: 123,
            status_info: Some(Default::default()),
        };

        let serialized = serde_json::to_string(&result).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ClearMonitoringResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let result_minimal = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Rejected,
            id: 0, // Valid
            status_info: None,
        };
        assert!(result_minimal.validate().is_ok());

        let result_positive_id = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Accepted,
            id: 456, // Valid
            status_info: Some(Default::default()),
        };
        assert!(result_positive_id.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_id() {
        let result = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Rejected,
            id: -1, // Invalid
            status_info: None,
        };
        let err = result.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["id"]);
    }
}
