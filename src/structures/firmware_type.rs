use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder, validate_string_length};
use crate::traits::OcppEntity;

/// Represents a copy of the firmware that can be loaded/updated on the Charging Station.
/// Used by: UpdateFirmwareRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FirmwareType {
    /// Required. URI defining the origin of the firmware.
    pub location: String,
    /// Required. Date and time at which the firmware shall be retrieved.
    pub retrieve_date_time: DateTime<Utc>,
    /// Optional. Date and time at which the firmware shall be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<DateTime<Utc>>,
    /// Optional. Certificate with which the firmware was signed. PEM encoded X.509 certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    /// Optional. Base64 encoded firmware signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl OcppEntity for FirmwareType {
    /// Validates the fields of FirmwareType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("location", 0, 2000, &self.location.chars());

        if let Some(signing_certificate) = &self.signing_certificate {
            e.check_cardinality("signing_certificate", 0, 5000, &signing_certificate.chars());
        }

        if let Some(signature) = &self.signature {
            e.check_cardinality("signature", 0, 800, &signature.chars());
        }

        e.build("FirmwareType")
    }
}