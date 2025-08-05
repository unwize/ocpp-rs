use serde::{Deserialize, Serialize};
use crate::enums::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;

/// A ChargingProfileCriterionType is a filter for charging profiles to be selected by a GetChargingProfilesRequest.
/// Used by: GetChargingProfilesRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingProfileCriterionType {
    /// Optional. Defines the purpose of the schedule transferred by this profile.
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    /// Optional. Value determining level in hierarchy stack of profiles.
    /// Higher values have precedence over lower values. Lowest level is 0.
    /// Constraints: 0 <= val
    pub stack_level: Option<i32>,
    /// Optional. List of all the chargingProfileIds requested. Any ChargingProfile that matches one of these profiles will be reported.
    /// If omitted, the Charging Station SHALL NOT filter on chargingProfileId.
    /// This field SHALL NOT contain more ids than set in ChargingProfileEntries.maxLimit.
    /// Cardinality 0..*, so represented as a Vec.
    pub charging_profile_id: Option<Vec<i32>>,
    /// Optional. For which charging limit sources, charging profiles SHALL be reported.
    /// If omitted, the Charging Station SHALL NOT filter on chargingLimitSource.
    /// Values defined in Appendix as ChargingLimitSourceEnumStringType.
    /// String length: 0..20
    /// Cardinality 0..4, so represented as a Vec.
    pub charging_limit_source: Option<Vec<String>>,
}

impl ChargingProfileCriterionType {
    /// Validates the fields of ChargingProfileCriterionType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate stack_level if present
        if let Some(level) = self.stack_level {
            if level < 0 {
                // println!("Validation failed: stack_level must be non-negative.");
                return false;
            }
        }

        // Validate charging_profile_id cardinality (no upper limit specified, but description mentions maxLimit)
        // Without `ChargingProfileEntries.maxLimit` definition, we can't enforce that specific constraint here.
        // We assume any number of IDs is valid for now, but a real implementation would need that context.

        // Validate charging_limit_source cardinality (max 4) and string length
        if let Some(sources) = &self.charging_limit_source {
            if sources.len() > 4 {
                // println!("Validation failed: charging_limit_source count exceeds 4.");
                return false;
            }
            for source in sources {
                if source.len() > 20 {
                    // println!("Validation failed: charging_limit_source string length exceeds 20.");
                    return false;
                }
            }
        }

        // No specific validation for 'charging_profile_purpose' without its type definition.

        true
    }
}

// Example usage (optional, for demonstration)
// TODO: Fix placeholders for charging_profile_purpose
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let criterion = ChargingProfileCriterionType {
            charging_profile_purpose: Some(ChargingProfilePurposeEnumType::TxProfile), // Placeholder
            stack_level: Some(1),
            charging_profile_id: Some(vec![101, 102]),
            charging_limit_source: Some(vec!["EMS".to_string(), "CSO".to_string()]),
        };

        let serialized = serde_json::to_string(&criterion).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingProfileCriterionType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(criterion, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let criterion_minimal = ChargingProfileCriterionType {
            charging_profile_purpose: None,
            stack_level: None,
            charging_profile_id: None,
            charging_limit_source: None,
        };
        assert!(criterion_minimal.validate());

        let criterion_full = ChargingProfileCriterionType {
            charging_profile_purpose: Some(ChargingProfilePurposeEnumType::TxProfile),
            stack_level: Some(0),
            charging_profile_id: Some(vec![1, 2, 3, 4, 5]), // Assuming no hard limit other than maxLimit
            charging_limit_source: Some(vec![
                "a".repeat(20),
                "b".repeat(20),
                "c".repeat(20),
                "d".repeat(20),
            ]),
        };
        assert!(criterion_full.validate());
    }

    #[test]
    fn test_validation_invalid_stack_level() {
        let criterion = ChargingProfileCriterionType {
            charging_profile_purpose: None,
            stack_level: Some(-1), // Invalid
            charging_profile_id: None,
            charging_limit_source: None,
        };
        assert!(!criterion.validate());
    }

    #[test]
    fn test_validation_too_many_charging_limit_sources() {
        let criterion = ChargingProfileCriterionType {
            charging_profile_purpose: None,
            stack_level: None,
            charging_profile_id: None,
            charging_limit_source: Some(vec![
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "s5".to_string(), // Too many
            ]),
        };
        assert!(!criterion.validate());
    }

    #[test]
    fn test_validation_charging_limit_source_too_long() {
        let criterion = ChargingProfileCriterionType {
            charging_profile_purpose: None,
            stack_level: None,
            charging_profile_id: None,
            charging_limit_source: Some(vec!["a".repeat(21)]), // Too long
        };
        assert!(!criterion.validate());
    }
}
