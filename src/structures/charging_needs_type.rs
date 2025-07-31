use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents the charging needs of an EV.
/// Used by: NotifyEVChargingNeedsRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingNeedsType {
    /// Required. Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType, // TODO: Implement EnergyTransferModeEnumType
    /// Optional. Modes of energy transfer that are marked as available by EV.
    /// Cardinality 0..*, so represented as a Vec.
    pub available_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>, // TODO: Implement EnergyTransferModeEnumType
    /// Optional. Indicates whether EV wants to operate in Dynamic or Scheduled mode.
    /// When absent, Scheduled mode is assumed for backwards compatibility.
    /// ISO 15118-20: ServiceSelectionReq(SelectedEnergyTransferService)
    pub control_mode: Option<ControlModeEnumType>, // TODO: Implement ControlModeEnumType
    /// Optional. Value of EVCC indicates that EV determines min/target SOC and departure time.
    /// A value of EVCC.SECC indicates that charging station or CSMS may also update min/target SOC and departure time.
    /// ISO 15118-20: ServiceSelectionReq(SelectedEnergyTransferService)
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>, // TODO: Implement MobilityNeedsModeEnumType
    /// Optional. Estimated departure time of the EV.
    /// ISO 15118-2: AC_EVChargeParameterType: DepartureTime
    /// ISO 15118-20: Dynamic/Scheduled_SEReqControlModeType: DepartureTime
    pub departure_time: Option<DateTime<Utc>>,
    /// Optional. The list of charging parameters that apply to an ISO 15118-20 session
    /// or any other session that supports bidirectional charging.
    pub v2x_charging_parameters: Option<V2XChargingParametersType>, // TODO: Implement V2XChargingParametersType
    /// Optional. EV DC charging parameters.
    pub dc_charging_parameters: Option<DCChargingParametersType>, // TODO: Implement DCChargingParametersType
    /// Optional. EV AC charging parameters.
    pub ac_charging_parameters: Option<ACChargingParametersType>, // TODO: Implement ACChargingParametersType
    /// Optional. Discharging and associated price offered by EV.
    /// EV schedule periods during which EV is willing to discharge have a negative value for power.
    pub ev_energy_offer: Option<EVEnergyOfferType>, // TODO: Implement EVEnergyOfferType
    /// Optional. Additional charging parameters for ISO 15118-20 AC bidirectional sessions with DER control (AC_BPT_DER).
    pub der_charging_parameters: Option<DERChargingParametersType>, // TODO: Implement DERChargingParametersType
}

impl ChargingNeedsType {
    /// Validates the fields of ChargingNeedsType.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // TODO: Implement logic once composite structs are defined
        true
    }
}
