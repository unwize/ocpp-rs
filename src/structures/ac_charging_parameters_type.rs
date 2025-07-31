use serde::{Deserialize, Serialize};

/// Represents AC charging parameters for ISO 15118-2.
/// Used by: Common::ChargingNeedsType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ACChargingParametersType {
    /// Required. Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to: ISO 15118-2: AC_EVChargeParameterType: EAmount
    /// ISO 15118-20: Dynamic/Scheduled_SEReqControlModeType: EVTargetEnergyRequest
    pub energy_amount: f64,
    /// Required. Minimum current (amps) supported by the electric vehicle (per phase).
    /// Relates to: ISO 15118-2: AC_EVChargeParameterType: EVMinCurrent
    pub ev_min_current: f64,
    /// Required. Maximum current (amps) supported by the electric vehicle (per phase). Includes cable.
    /// Relates to: ISO 15118-2: AC_EVChargeParameterType: EVMaxCurrent
    pub ev_max_current: f64,
    /// Required. Maximum voltage supported by vehicle.
    /// Relates to: ISO 15118-2: AC_EVChargeParameterType: EVMaxVoltage
    pub ev_max_voltage: f64,
}