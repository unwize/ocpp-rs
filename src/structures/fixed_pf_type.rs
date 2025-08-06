use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Used by: Common::FixedPFGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FixedPFType {
    /// Required. Priority of setting (0=highest)
    pub priority: i32,
    /// Required. Power factor, cos(phi), as value between 0..1.
    pub displacement: f64,
    /// Required. True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited).
    pub excitation: bool,
    /// Optional. Time when this setting becomes active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. Duration in seconds that this setting is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl OcppEntity for FixedPFType {
    /// Validates the fields of FixedPFType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);
        e.check_bounds("displacement", 0.0, 1.0, self.displacement);

        if let Some(duration) = self.duration {
            e.check_bounds("duration", 0.0, f64::MAX, duration);
        }

        e.build("FixedPFType")
    }
}