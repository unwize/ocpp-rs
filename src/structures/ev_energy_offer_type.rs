use crate::errors::OcppError;
use crate::structures::ev_absolute_price_schedule_type::EVAbsolutePriceScheduleType;
use crate::structures::ev_power_schedule_type::EVPowerScheduleType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// A schedule of the energy amount over time that EV is willing to discharge.
/// A negative value indicates the willingness to discharge under specific conditions,
/// a positive value indicates that the EV currently is not able to offer energy to discharge.
/// Used by: Common::ChargingNeedsType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EVEnergyOfferType {
    /// Required. Power schedule offered for discharging.
    pub ev_power_schedule: EVPowerScheduleType,
    /// Optional. Price schedule for which EV is willing to discharge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_absolute_price_schedule: Option<EVAbsolutePriceScheduleType>,
}

impl OcppEntity for EVEnergyOfferType {
    /// Validates the fields of EVEnergyOfferType.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        if let Err(e) = self.ev_power_schedule.validate() {
            errors.push(e);
        }

        if let Some(ev_absolute_price_schedule) = &self.ev_absolute_price_schedule {
            if let Err(e) = ev_absolute_price_schedule.validate() {
                errors.push(e);
            }
        }

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EVEnergyOfferType".to_string(),
                related: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let energy_offer = EVEnergyOfferType {
            ev_power_schedule: EVPowerScheduleType::default(), // Placeholder
            ev_absolute_price_schedule: Some(EVAbsolutePriceScheduleType {
                time_anchor: Default::default(),
                currency: "USD".to_string(),
                price_algorithm: "".to_string(),
                ev_absolute_price_schedule_entries: vec![],
            }),
        };

        let serialized = serde_json::to_string(&energy_offer).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVEnergyOfferType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(energy_offer, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let energy_offer_minimal = EVEnergyOfferType {
            ev_power_schedule: EVPowerScheduleType::default(),
            ev_absolute_price_schedule: None,
        };
        assert!(energy_offer_minimal.validate().is_ok());

        let energy_offer_full = EVEnergyOfferType {
            ev_power_schedule: EVPowerScheduleType::default(),
            ev_absolute_price_schedule: Some(EVAbsolutePriceScheduleType {
                time_anchor: Default::default(),
                currency: "USD".to_string(),
                price_algorithm: "".to_string(),
                ev_absolute_price_schedule_entries: vec![],
            }),
        };
        assert!(energy_offer_full.validate().is_ok());
    }
}
