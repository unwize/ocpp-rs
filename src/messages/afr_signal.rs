use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::generic_status_enum_type::GenericStatusEnumType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.2. AFRRSignal
pub struct AFRRSignal;

impl OcppMessage for AFRRSignal {
    type Request = AFRRSignalRequest;
    type Response = AFRRSignalResponse;
}

/// 1.2.1. AFRRSignalRequest
/// This message passes an aFRR signal on to the charging station. Charging station uses the value of signal to select a
/// matching power value from the v2xSignalWattCurve in the ChargingSchedulePeriod.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalRequest {
    /// Required. Time when signal becomes active.
    pub timestamp: DateTime<Utc>,
    /// Required. Value of signal in v2xSignalWattCurve.
    pub signal: i32,
}

impl OcppEntity for AFRRSignalRequest {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

/// 1.2.2. AFRRSignalResponse
/// Response stating whether signal was accepted. Response will be Accepted if a v2xSignalWattCurve_ element exists in the
/// ChargingSchedulePeriodType for that point in time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalResponse {
    /// Required.
    pub status: GenericStatusEnumType,
    /// Optional. Additional information on status.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for AFRRSignalResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("status", &self.status);
        if let Some(status_info) = &self.status_info {
            b.check_member("statusInfo", status_info);
        }

        b.build("AFRRSignalResponse")
    }
}