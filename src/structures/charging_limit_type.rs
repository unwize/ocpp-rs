use serde::{Deserialize, Serialize};

/// Represents a charging limit.
/// Used by: NotifyChargingLimitRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingLimitType {
    /// Required. Represents the source of the charging limit.
    /// Values defined in appendix as ChargingLimitSourceEnumStringType.
    /// String length: 0..20
    pub charging_limit_source: String,
    /// Optional. True when the reported limit concerns local generation that is providing extra capacity,
    /// instead of a limitation.
    pub is_local_generation: Option<bool>,
    /// Optional. Indicates whether the charging limit is critical for the grid.
    pub is_grid_critical: Option<bool>,
}

impl ChargingLimitType {
    /// Validates the fields of ChargingLimitType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate charging_limit_source length
        if self.charging_limit_source.len() > 20 {
            // println!("Validation failed: charging_limit_source length exceeds 20.");
            return false;
        }

        true
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let charging_limit = ChargingLimitType {
            charging_limit_source: "EMS".to_string(),
            is_local_generation: Some(true),
            is_grid_critical: Some(false),
        };

        let serialized = serde_json::to_string(&charging_limit).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingLimitType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(charging_limit, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let charging_limit_minimal = ChargingLimitType {
            charging_limit_source: "CSO".to_string(),
            is_local_generation: None,
            is_grid_critical: None,
        };
        assert!(charging_limit_minimal.validate());

        let charging_limit_full = ChargingLimitType {
            charging_limit_source: "a".repeat(20),
            is_local_generation: Some(true),
            is_grid_critical: Some(true),
        };
        assert!(charging_limit_full.validate());
    }

    #[test]
    fn test_validation_source_too_long() {
        let charging_limit = ChargingLimitType {
            charging_limit_source: "a".repeat(21), // Too long
            is_local_generation: None,
            is_grid_critical: None,
        };
        assert!(!charging_limit.validate());
    }
}
