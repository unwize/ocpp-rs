use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalUsageType {
    /// Required. Total energy usage.
    pub energy: f64,
    /// Required. Total duration of the charging session (including the duration of charging and not charging), in seconds.
    pub charging_time: i32,
    /// Required. Total duration of the charging session where the EV was not charging (no energy was transferred between EVSE and EV), in seconds.
    pub idle_time: i32,
    /// Optional. Total time of reservation in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<i32>,
}

impl Default for TotalUsageType {
    fn default() -> TotalUsageType {
        Self {
            energy: 0.0,
            charging_time: 0,
            idle_time: 0,
            reservation_time: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for TotalUsageType {
    /// Validates the fields of TotalUsageType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("energy", 0.0, f64::MAX, self.energy);
        e.check_bounds("charging_time", 0, i32::MAX, self.charging_time);
        e.check_bounds("idle_time", 0, i32::MAX, self.idle_time);

        if let Some(reservation_time) = &self.reservation_time {
            e.check_bounds("reservation_time", 0, i32::MAX, *reservation_time);
        }

        e.build("TotalUsageType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TotalUsageType {
        TotalUsageType {
            energy: 15.5,
            charging_time: 3600,
            idle_time: 120,
            reservation_time: Some(600),
        }
    }

    #[test]
    fn test_validate_success_with_reservation_time() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_reservation_time() {
        let mut data = create_test_instance();
        data.reservation_time = None;
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_energy() {
        let mut data = create_test_instance();
        data.energy = -1.0;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_charging_time() {
        let mut data = create_test_instance();
        data.charging_time = -10;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_idle_time() {
        let mut data = create_test_instance();
        data.idle_time = -5;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_reservation_time() {
        let mut data = create_test_instance();
        data.reservation_time = Some(-30);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TotalUsageType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
