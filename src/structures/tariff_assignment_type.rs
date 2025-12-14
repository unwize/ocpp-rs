use crate::enums::tariff_kind_enum_type::TariffKindEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Shows assignment of tariffs to EVSE or IdToken.
/// Used by: GetTariffsResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    /// Required. Tariff identifier.
    pub tariff_id: String,
    /// Required. Kind of tariff (driver/default).
    pub tariff_kind: TariffKindEnumType,
    /// Optional. Date/time when this tariff becomes active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. List of EVSE Ids to which the tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_ids: Option<Vec<i32>>,
    /// Optional. List of IdTokens related to the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tokens: Option<Vec<String>>,
}

impl Default for TariffAssignmentType {
    fn default() -> TariffAssignmentType {
        Self {
            tariff_id: "".to_string(),
            tariff_kind: TariffKindEnumType::DefaultTariff,
            valid_from: None,
            evse_ids: None,
            id_tokens: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for TariffAssignmentType {
    /// Validates the fields of TariffAssignmentType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("tariff_id", 0, 60, &self.tariff_id.chars());

        // Manually check bounds for each i32 since that type does not implement OcppEntity and has no `validate` function
        // for use with `check_iter_member`.
        if let Some(evse_ids) = &self.evse_ids {
            for i in 0..evse_ids.len() {
                e.check_bounds(format!("evse_ids[{i}]").as_str(), 0, i32::MAX, evse_ids[i]);
            }
        }

        // Manually check String length since it does not implement OcppEntity and has no `validate` function for use
        // with `check_iter_member`
        if let Some(id_tokens) = &self.id_tokens {
            for i in 0..id_tokens.len() {
                e.check_cardinality(
                    format!("id_tokens[{i}]").as_str(),
                    0,
                    255,
                    &id_tokens[i].chars(),
                );
            }
        }

        e.build("TariffAssignmentType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::tariff_kind_enum_type::TariffKindEnumType;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = TariffAssignmentType {
            tariff_id: "T123".to_string(),
            tariff_kind: TariffKindEnumType::DefaultTariff,
            valid_from: None,
            evse_ids: Some(vec![1, 2]),
            id_tokens: Some(vec!["token1".to_string(), "token2".to_string()]),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_tariff_id_too_long() {
        let mut data = TariffAssignmentType::default();
        data.tariff_id = "a".repeat(61);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_failure_negative_evse_id() {
        let mut data = TariffAssignmentType::default();
        data.tariff_id = "T123".to_string();
        data.evse_ids = Some(vec![-1]);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_failure_id_token_too_long() {
        let mut data = TariffAssignmentType::default();
        data.tariff_id = "T123".to_string();
        data.id_tokens = Some(vec!["a".repeat(256)]);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TariffAssignmentType {
            tariff_id: "T123".to_string(),
            tariff_kind: TariffKindEnumType::DefaultTariff,
            valid_from: Some(Utc::now()),
            evse_ids: Some(vec![1, 2]),
            id_tokens: Some(vec!["token1".to_string()]),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffAssignmentType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
