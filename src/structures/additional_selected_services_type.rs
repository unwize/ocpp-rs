use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Represents additional selected services as part of the ISO 15118-20 price schedule.
/// Used by: Common::AbsolutePriceScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct AdditionalSelectedServicesType {
    /// Required. Human-readable string to identify this service.
    /// String length: 0..80
    pub service_name: String,
    /// Required. Cost of the service.
    /// Adapted from RationalNumberType, considered as u32.
    pub service_fee: u32,
}

impl OcppEntity for AdditionalSelectedServicesType {
    /// Validates the fields of AdditionalSelectedServicesType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_cardinality("service_name", 0, 80, &self.service_name.chars());
        e.build("AdditionalSelectedServicesType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let service = AdditionalSelectedServicesType {
            service_name: "Charging Service".to_string(),
            service_fee: 1500, // Example fee
        };

        let serialized = serde_json::to_string(&service).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: AdditionalSelectedServicesType =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(service, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let service = AdditionalSelectedServicesType {
            service_name: "Short service name".to_string(),
            service_fee: 100,
        };
        assert!(service.validate().is_ok());

        let service_max_len = AdditionalSelectedServicesType {
            service_name: "a".repeat(80),
            service_fee: 0, // Valid fee
        };
        assert!(service_max_len.validate().is_ok());
    }

    #[test]
    fn test_validation_service_name_too_long() {
        let service = AdditionalSelectedServicesType {
            service_name: "a".repeat(81), // Too long
            service_fee: 500,
        };
        assert!(service.validate().is_err());
    }
}
