use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::enums::control_model_enum_type::ControlModeEnumType;
use crate::enums::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;
use crate::structures::ac_charging_parameters_type::ACChargingParametersType;
use crate::structures::dc_charging_parameters_type::DCChargingParametersType;
use crate::structures::der_charging_parameters_type::DERChargingParametersType;
use crate::structures::ev_energy_offer_type::EVEnergyOfferType;

/// Represents the charging needs of an EV.
/// Used by: NotifyEVChargingNeedsRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingNeedsType {
    /// Required. Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,
    /// Optional. Modes of energy transfer that are marked as available by EV.
    /// Cardinality 0..*, so represented as a Vec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,
    /// Optional. Indicates whether EV wants to operate in Dynamic or Scheduled mode.
    /// When absent, Scheduled mode is assumed for backwards compatibility.
    /// ISO 15118-20: ServiceSelectionReq(SelectedEnergyTransferService)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_mode: Option<ControlModeEnumType>,
    /// Optional. Value of EVCC indicates that EV determines min/target SOC and departure time.
    /// A value of EVCC.SECC indicates that charging station or CSMS may also update min/target SOC and departure time.
    /// ISO 15118-20: ServiceSelectionReq(SelectedEnergyTransferService)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>, // TODO: Implement MobilityNeedsModeEnumType
    /// Optional. Estimated departure time of the EV.
    /// ISO 15118-2: AC_EVChargeParameterType: DepartureTime
    /// ISO 15118-20: Dynamic/Scheduled_SEReqControlModeType: DepartureTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,
    /// Optional. The list of charging parameters that apply to an ISO 15118-20 session
    /// or any other session that supports bidirectional charging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_charging_parameters: Option<V2XChargingParametersType>, // TODO: Implement V2XChargingParametersType
    /// Optional. EV DC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
    /// Optional. EV AC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,
    /// Optional. Discharging and associated price offered by EV.
    /// EV schedule periods during which EV is willing to discharge have a negative value for power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_offer: Option<EVEnergyOfferType>,
    /// Optional. Additional charging parameters for ISO 15118-20 AC bidirectional sessions with DER control (AC_BPT_DER).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub der_charging_parameters: Option<DERChargingParametersType>,
}

impl ChargingNeedsType {
    /// Validates the fields of ChargingNeedsType.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // TODO: Implement logic once composite structs are defined
        true
    }
}
