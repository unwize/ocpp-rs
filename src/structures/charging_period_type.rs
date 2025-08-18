use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::cost_dimension_type::CostDimensionType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A ChargingPeriodType consists of a start time, and a list of possible values that influence this period,
/// for example: amount of energy charged this period, maximum current during this period etc.
/// Used by: Common::CostDetailsType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingPeriodType {
    /// Optional. Unique identifier of the Tariff that was used to calculate cost.
    /// If not provided, then cost was calculated by some other means.
    /// String length: 0..60
    pub tariff_id: Option<String>,
    /// Required. Start timestamp of charging period. A period ends when the next period starts.
    /// The last period ends when the session ends.
    pub start_period: DateTime<Utc>,
    /// Optional. List of volume per cost dimension for this charging period.
    /// Cardinality 0..*, so represented as a Vec.
    pub dimensions: Option<Vec<CostDimensionType>>,
}

impl Default for ChargingPeriodType {
    fn default() -> ChargingPeriodType {
        Self {
            tariff_id: None,
            start_period: Utc::now(),
            dimensions: None,
        }
    }
}

impl OcppEntity for ChargingPeriodType {
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(tariff_id) = &self.tariff_id {
            e.check_cardinality("tariff_id", 0, 60, &tariff_id.chars());
        }

        if let Some(dimensions) = &self.dimensions {
            e.check_cardinality("start_period", 0, 60, &dimensions.iter());
            e.check_iter_member("dimensions", dimensions.iter());
        }

        e.build("ChargingPeriodType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let charging_period = ChargingPeriodType {
            tariff_id: Some("TARIFF_ABC_123".to_string()),
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 17, 0, 0).unwrap(),
            dimensions: Some(vec![]), // TODO: Add actual values
        };

        let serialized = serde_json::to_string(&charging_period).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingPeriodType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(charging_period, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let charging_period_minimal = ChargingPeriodType {
            tariff_id: None,
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 17, 0, 0).unwrap(),
            dimensions: None,
        };
        assert!(charging_period_minimal.validate().is_ok());

        let charging_period_full = ChargingPeriodType {
            tariff_id: Some("a".repeat(60)),
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 18, 0, 0).unwrap(),
            dimensions: Some(vec![]), // TODO: Add actual values
        };
        assert!(charging_period_full.validate().is_ok());
    }

    #[test]
    fn test_validation_tariff_id_too_long() {
        let charging_period = ChargingPeriodType {
            tariff_id: Some("a".repeat(61)), // Too long
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 17, 0, 0).unwrap(),
            dimensions: None,
        };
        assert!(charging_period.validate().is_err());
    }
}
