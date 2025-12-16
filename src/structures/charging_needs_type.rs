use crate::enums::control_model_enum_type::ControlModeEnumType;
use crate::enums::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;
use crate::enums::mobility_needs_mode_enum_type::MobilityNeedsModeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::ac_charging_parameters_type::ACChargingParametersType;
use crate::structures::dc_charging_parameters_type::DCChargingParametersType;
use crate::structures::der_charging_parameters_type::DERChargingParametersType;
use crate::structures::ev_energy_offer_type::EVEnergyOfferType;
use crate::structures::v2x_charging_parameters_type::V2XChargingParametersType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the charging needs of an EV.
/// Used by: NotifyEVChargingNeedsRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>,
    /// Optional. Estimated departure time of the EV.
    /// ISO 15118-2: AC_EVChargeParameterType: DepartureTime
    /// ISO 15118-20: Dynamic/Scheduled_SEReqControlModeType: DepartureTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,
    /// Optional. The list of charging parameters that apply to an ISO 15118-20 session
    /// or any other session that supports bidirectional charging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_charging_parameters: Option<V2XChargingParametersType>,
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
#[typetag::serde]
impl OcppEntity for ChargingNeedsType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(v2x_charging_parameters) = &self.v2x_charging_parameters {
            e.check_member("v2x_charging_parameters", v2x_charging_parameters);
        }

        if let Some(dc_charging_parameters) = &self.dc_charging_parameters {
            e.check_member("dc_charging_parameters", dc_charging_parameters);
        }

        if let Some(ac_charging_parameters) = &self.ac_charging_parameters {
            e.check_member("ac_charging_parameters", ac_charging_parameters);
        }

        if let Some(ev_energy_offer) = &self.ev_energy_offer {
            e.check_member("ev_energy_offer", ev_energy_offer);
        }

        if let Some(der_charging_parameters) = &self.der_charging_parameters {
            e.check_member("der_charging_parameters", der_charging_parameters);
        }

        e.build("ChargingNeedsType")
    }
}
