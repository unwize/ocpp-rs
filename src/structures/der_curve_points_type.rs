use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::errors::OcppError;
use crate::traits::OcppEntity;

/// DERCurvePointsType is used by: Common::DERCurveType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DERCurvePointsType {
    /// Required. The data value of the X-axis (independent) variable, depending on the curve type.
    pub x: f64, // decimal
    /// Required. The data value of the Y-axis (dependent) variable, depending on the DERUnitEnumType of the curve.
    /// If y is power factor, then a positive value means DER is absorbing reactive power (under-excited),
    /// a negative value when DER is injecting reactive power (over-excited).
    pub y: f64, // decimal
}

impl OcppEntity for DERCurvePointsType {
    /// Validates the fields of DERCurvePointsType.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

impl DERCurvePointsType {
    fn new(x: f64, y: f64) -> Self {
        DERCurvePointsType { x, y }
    }
}

impl Into<String> for DERCurvePointsType {
    fn into(self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

impl Display for DERCurvePointsType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for DERCurvePointsType {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let curve_point = DERCurvePointsType {
            x: 10.5,
            y: 20.7,
        };

        let serialized = serde_json::to_string(&curve_point).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: DERCurvePointsType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(curve_point, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let curve_point_positive = DERCurvePointsType {
            x: 1.0,
            y: 2.0,
        };
        assert!(curve_point_positive.validate().is_ok());

        let curve_point_negative_y = DERCurvePointsType {
            x: 5.0,
            y: -0.8, // Valid for power factor
        };
        assert!(curve_point_negative_y.validate().is_ok());

        let curve_point_zero = DERCurvePointsType {
            x: 0.0,
            y: 0.0,
        };
        assert!(curve_point_zero.validate().is_ok());
    }
}
