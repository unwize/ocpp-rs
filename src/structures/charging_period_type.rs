use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A ChargingPeriodType consists of a start time, and a list of possible values that influence this period,
/// for example: amount of energy charged this period, maximum current during this period etc.
/// Used by: Common::CostDetailsType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub dimensions: Option<Vec<CostDimensionType>>, // TODO: Implement CostDimensionType
}

impl ChargingPeriodType {
    /// Validates the fields of ChargingPeriodType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate tariff_id length if present
        if let Some(tariff_id) = &self.tariff_id {
            if tariff_id.len() > 60 {
                // println!("Validation failed: tariff_id length exceeds 60.");
                return false;
            }
        }

        // TODO: Validate CostDimensionType

        true
    }
}

// Example usage (optional, for demonstration)
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
        assert!(charging_period_minimal.validate());

        let charging_period_full = ChargingPeriodType {
            tariff_id: Some("a".repeat(60)),
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 18, 0, 0).unwrap(),
            dimensions: Some(vec![]), // TODO: Add actual values
        };
        assert!(charging_period_full.validate());
    }

    #[test]
    fn test_validation_tariff_id_too_long() {
        let charging_period = ChargingPeriodType {
            tariff_id: Some("a".repeat(61)), // Too long
            start_period: Utc.with_ymd_and_hms(2025, 7, 31, 17, 0, 0).unwrap(),
            dimensions: None,
        };
        assert!(!charging_period.validate());
    }
}
