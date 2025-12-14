use crate::enums::battery_swap_event_enum_types::BatterySwapEventEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::battery_data_type::BatteryDataType;
use crate::structures::id_token_type::IdTokenType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.4. BatterySwap
pub struct BatterySwap;

impl OcppMessage for BatterySwap {
    type Request = BatterySwapRequest;
    type Response = BatterySwapResponse;
}

/// 1.4.1. BatterySwapRequest
/// Message sent by Charging Station when a battery is swapped in or out of a battery swap station.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    /// Required. Battery in/out
    pub event_type: BatterySwapEventEnumType,
    /// Required. RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    pub request_id: i32,
    /// Required. Id token of EV Driver
    pub id_token: IdTokenType,
    /// Required. Info on batteries inserted or taken out.
    pub battery_data: Vec<BatteryDataType>,
}
#[typetag::serde]
impl OcppEntity for BatterySwapRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("id_token", &self.id_token);

        b.check_cardinality("battery_data", 1, usize::MAX, &self.battery_data.iter());
        b.check_iter_member("battery_data", self.battery_data.iter());

        b.build("BatterySwapRequest")
    }
}

/// 1.4.2. BatterySwapResponse
/// Empty response by CSMS to confirm receipt of BatterySwapRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapResponse {}

#[typetag::serde]
impl OcppEntity for BatterySwapResponse {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}
