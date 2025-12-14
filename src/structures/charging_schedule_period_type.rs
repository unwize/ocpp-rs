use crate::enums::operation_mode_enum_type::OperationModeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::v2x_freq_watt_point_type::V2XFreqWattPointType;
use crate::structures::v2x_signal_watt_point_type::V2XSignalWattPointType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Charging schedule period structure defines a time period in a charging schedule.
/// It is used in: CompositeScheduleType and in ChargingScheduleType.
/// When used in a NotifyEVChargingScheduleRequest only startPeriod, limit, limit_L2, limit_L3 are relevant.
/// Used by: Common::ChargingScheduleType, Common::CompositeScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingSchedulePeriodType {
    /// Required. Start of the period, in seconds from the start of schedule.
    /// The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,

    /// Optional. Only when not required by the ChargingRateUnit or ChargingRateUnit.Setpoint.
    /// Internal.evse.LocalFrequency, Local.GridBalancing and Local.PeakShaving are examples of where this is
    /// applicable ChargingRateUnit. This SHOULD BE a non-negative value. A negative value is interpreted as the
    /// negative value to specify a discharging limit. When using ChargingRateUnit.Current, this field represents
    /// the power of all phases, unless values are provided for L2 and L3. In which case this field represents L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,

    /// Optional. Charging rate limit on phase L2 in the applicable ChargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f64>,

    /// Optional. Charging rate limit on phase L3 in the applicable ChargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f64>,

    /// Optional. The number of phases that can be used for charging.
    /// For an EVSE this field should be omitted.
    /// For an AC EVSE: a default value of numberOfPhases=3 will be assumed if this field is omitted.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,

    /// Optional. Values: 1, 3. Used if numberOfPhases=1 and if the EVSE supports phase switching.
    /// In case of a single phase EV, i.e. ACPhaseSwitchingSupported is defined and true.
    /// If ACPhaseSwitchingSupported is defined and true, and phaseToUse is omitted, the Charging Station
    /// SHALL assume that the EV can switch phases. If both conditions are true, and phaseToUse is omitted,
    /// the Charging Station SHALL assume that the EV can switch phases.
    /// Constraints: 1 <= val <= 3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,

    /// Optional. Limit in ChargingRateUnit that the EV is allowed to discharge with.
    /// Note, these are negative values. A negative value means that the power or current can be positive and negative.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit: Option<f64>,

    /// Optional. Limit in ChargingRateUnit on phase L2 that the EV is allowed to discharge with.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit_l2: Option<f64>,

    /// Optional. Limit in ChargingRateUnit on phase L3 that the EV is allowed to discharge with.
    /// Constraints: val <= 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit_l3: Option<f64>,

    /// Optional. Setpoint in ChargingRateUnit that the EV is allowed to discharge with.
    /// Note, these are negative values. A negative value means that the power or current can be positive and negative.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint: Option<f64>,

    /// Optional. Setpoint in ChargingRateUnit that the EV should follow on phase L2 as closely as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_l2: Option<f64>,

    /// Optional. Setpoint in ChargingRateUnit that the EV should follow on phase L3 as closely as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_l3: Option<f64>,

    /// Optional. Setpoint for reactive power (or current) in ChargingRateUnit that the EV should follow.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive: Option<f64>,

    /// Optional. Setpoint for reactive power (or current) in ChargingRateUnit that the EV should follow on phase L2 as closely as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive_l2: Option<f64>,

    /// Optional. (2.1) Setpoint for reactive power (or current) in
    /// chargingRateUnit that the EV should follow on phase L3
    /// as closely as possible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive_l3: Option<f64>,

    /// Optional. (2.1) If true, the EV should attempt to keep the
    /// BMS preconditioned for this time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precondition_request: Option<bool>,

    /// Optional. (2.1) If true, the EVSE must turn off power
    /// electronics/modules associated with this transaction.
    /// Default value when absent is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_sleep: Option<bool>,

    /// Optional. (2.1) Power value that, when present, is used as
    /// a baseline on top of which values from v2xFreqWattCurve
    /// and v2xSignalWattCurve are added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_baseline: Option<f64>,

    /// Optional. (2.1) Charging operation mode to use during
    /// this time interval. When absent defaults to
    /// ChargingOnly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_mode: Option<OperationModeEnumType>,

    /// Optional. (2.1) Only required when operationMode = LocalFrequency.
    /// When used it must contain at least two coordinates to specify a
    /// power-frequency table to use during this period. The table determines
    /// the value of setpoint power for a given frequency. chargingRateUnit
    /// must be W for LocalFrequency control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_freq_watt_curve: Option<Vec<V2XFreqWattPointType>>,

    /// Optional. (2.1) Only used, but not required, when
    /// operationMode = LocalFrequency. When used it must
    /// contain at least two coordinates to specify a signal-
    /// frequency curve to use during this period. The curve
    /// determines the value of setpoint power for a given signal.
    /// chargingRateUnit must be W for LocalFrequency
    /// control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_signal_watt_point_type: Option<Vec<V2XSignalWattPointType>>,
}

impl Default for ChargingSchedulePeriodType {
    fn default() -> ChargingSchedulePeriodType {
        Self {
            start_period: 0,
            limit: None,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            phase_to_use: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for ChargingSchedulePeriodType {
    /// Validates the fields of ChargingSchedulePeriodType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(number_phases) = self.number_phases {
            e.check_bounds("number_phases", 1, 3, number_phases);
        }

        if let Some(phase_to_use) = self.phase_to_use {
            e.check_bounds("phase_to_use", 1, 3, phase_to_use);
        }

        if let Some(discharge_limit) = self.discharge_limit {
            e.check_bounds("discharge_limit", f64::MIN, 0.0, discharge_limit);
        }
        if let Some(discharge_limit_l2) = self.discharge_limit_l2 {
            e.check_bounds("discharge_limit_l2", f64::MIN, 0.0, discharge_limit_l2);
        }
        if let Some(discharge_limit_l3) = self.discharge_limit_l3 {
            e.check_bounds("discharge_limit_l3", f64::MIN, 0.0, discharge_limit_l3);
        }

        if let Some(v2x_freq_watt_curve) = &self.v2x_freq_watt_curve {
            e.check_cardinality("v2x_freq_watt_curve", 0, 20, &v2x_freq_watt_curve.iter());
        }

        if let Some(v2x_signal_watt_curve) = &self.v2x_signal_watt_point_type {
            e.check_cardinality(
                "v2x_signal_watt_curve",
                0,
                20,
                &v2x_signal_watt_curve.iter(),
            );
        }

        e.build("ChargingSchedulePeriodType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let period = ChargingSchedulePeriodType {
            start_period: 0,
            limit: Some(16.0),
            limit_l2: Some(10.0),
            limit_l3: Some(10.0),
            number_phases: Some(3),
            phase_to_use: Some(1),
            discharge_limit: Some(-5.0),
            discharge_limit_l2: Some(-3.0),
            discharge_limit_l3: Some(-2.0),
            setpoint: Some(10.0),
            setpoint_l2: Some(5.0),
            setpoint_l3: Some(5.0),
            setpoint_reactive: Some(2.0),
            setpoint_reactive_l2: Some(1.0),
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };

        let serialized = serde_json::to_string(&period).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingSchedulePeriodType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(period, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let period_minimal = ChargingSchedulePeriodType {
            start_period: 0,
            limit: None,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            phase_to_use: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period_minimal.validate().is_ok());

        let period_with_limits = ChargingSchedulePeriodType {
            start_period: 3600,
            limit: Some(20.0),
            limit_l2: Some(15.0),
            limit_l3: Some(15.0),
            number_phases: Some(1),
            phase_to_use: Some(3),
            discharge_limit: Some(-10.0),
            discharge_limit_l2: Some(-8.0),
            discharge_limit_l3: Some(-7.0),
            setpoint: Some(12.0),
            setpoint_l2: Some(6.0),
            setpoint_l3: Some(6.0),
            setpoint_reactive: Some(3.0),
            setpoint_reactive_l2: Some(1.5),
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period_with_limits.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_number_phases() {
        let period = ChargingSchedulePeriodType {
            start_period: 0,
            number_phases: Some(-1), // Invalid
            limit: None,
            limit_l2: None,
            limit_l3: None,
            phase_to_use: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_phase_to_use() {
        let period_low = ChargingSchedulePeriodType {
            start_period: 0,
            phase_to_use: Some(0), // Invalid
            limit: None,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period_low.validate().is_err());

        let period_high = ChargingSchedulePeriodType {
            start_period: 0,
            phase_to_use: Some(4), // Invalid
            limit: None,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period_high.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_discharge_limit() {
        let period = ChargingSchedulePeriodType {
            start_period: 0,
            discharge_limit: Some(1.0), // Invalid (must be <= 0)
            limit: None,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            phase_to_use: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            precondition_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_point_type: None,
        };
        assert!(period.validate().is_err());
    }
}
