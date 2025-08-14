use crate::enums::tariff_clear_status_enum_type::TariffClearStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Result of a clear tariffs request.
/// Used by: ClearTariffsResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClearTariffsResultType {
    /// Optional. Id of tariff for which status is reported.
    /// If no tariffs were found, then this field is absent, and status will be NoTariff.
    /// String length: 0..60
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_id: Option<String>,
    /// Required.
    pub status: TariffClearStatusEnumType,
    /// Optional. Additional info on status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for ClearTariffsResultType {
    /// Validates the fields of ClearTariffsResultType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Validate tariff_id length
        if let Some(tariff_id) = &self.tariff_id {
            e.check_cardinality("tariff_id", 0, 60, &tariff_id.chars());
        }

        if let Some(status_info) = &self.status_info {
            e.check_member("status_info", status_info);
        }

        e.build("ClearTariffsResultType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let result = ClearTariffsResultType {
            tariff_id: Some("TARIFF_XYZ_789".to_string()),
            status: TariffClearStatusEnumType::Accepted,
            status_info: Some(Default::default()), // Placeholder
        };

        let serialized = serde_json::to_string(&result).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ClearTariffsResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let result_minimal = ClearTariffsResultType {
            tariff_id: None,
            status: TariffClearStatusEnumType::Accepted,
            status_info: None,
        };
        assert!(result_minimal.validate().is_ok());

        let result_with_id = ClearTariffsResultType {
            tariff_id: Some("a".repeat(60)), // Valid length
            status: TariffClearStatusEnumType::Accepted,
            status_info: Some(Default::default()),
        };
        assert!(result_with_id.validate().is_ok());
    }

    #[test]
    fn test_validation_tariff_id_too_long() {
        let result = ClearTariffsResultType {
            tariff_id: Some("a".repeat(61)), // Invalid
            status: TariffClearStatusEnumType::Accepted,
            status_info: None,
        };
        let err = result.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["tariff_id"]);
    }
}
