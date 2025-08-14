use crate::enums::der_control_enum_type::DERControlEnumType;
use crate::enums::islanding_detection_enum_type::IslandingDetectionEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// DER DC charging parameters for ISO 15118-2
/// Used by: Common::ChargingNeedsType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DERChargingParametersType {
    /// Optional. DER control functions supported by EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType:DERControlFunctions (bitmap)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_supported_der_control: Option<Vec<DERControlEnumType>>,

    /// Optional. Rated maximum injected active power by EV, at specified over-excited power factor (overExcitedPowerFactor)
    /// It can also be defined as the rated maximum discharge power at the rated minimum injected reactive power value.
    /// This means that if the EV is providing reactive power support, and it is requested to discharge at max power (e.g. to satisfy an EMS request),
    /// the EV may override the request and discharge up to evOverExcitedMaximumDischargePower to meet the minimum reactive power requirements.
    /// Corresponds to the WOvPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVOverExcitedMaximumDischargePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_over_excited_max_discharge_power: Option<f64>, // decimal

    /// Optional. EV power factor when injecting (over excited) the minimum reactive power.
    /// Corresponds to the OvPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVOverExcitedPowerFactor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_over_excited_power_factor: Option<f64>, // decimal

    /// Optional. Rated maximum injected active power by EV supported at specified under-excited power factor (evUnderExcitedPowerFactor)
    /// It can also be defined as the rated maximum discharge power at the rated minimum absorbed reactive power value.
    /// This means that if the EV is providing reactive power support, and it is requested to discharge at max power (e.g. to satisfy an EMS request),
    /// the EV may override the request and discharge up to evUnderExcitedMaximumDischargePower to meet the minimum reactive power requirements.
    /// Corresponds to the WUnPF attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVUnderExcitedMaximumDischargePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_under_excited_max_discharge_power: Option<f64>, // decimal

    /// Optional. EV power factor when injecting (under excited) the minimum reactive power.
    /// Corresponds to the UnPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVUnderExcitedPowerFactor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_under_excited_power_factor: Option<f64>, // decimal

    /// Optional. Rated maximum total apparent power, defined by min(EV, EVSE) in va.
    /// Corresponds to the VAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumApparentPower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_apparent_power: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed apparent power, defined by min(EV, EVSE) in va.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the ChAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_apparent_power: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed apparent power on phase L2, defined by min(EV, EVSE) in va.
    /// Corresponds to the ChAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_apparent_power_l2: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed apparent power on phase L3, defined by min(EV, EVSE) in va.
    /// Corresponds to the ChAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_apparent_power_l3: Option<f64>, // decimal

    /// Optional. Rated maximum injected apparent power, defined by min(EV, EVSE) in va.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_apparent_power: Option<f64>, // decimal

    /// Optional. Rated maximum injected apparent power on phase L2, defined by min(EV, EVSE) in va.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_apparent_power_l2: Option<f64>, // decimal

    /// Optional. Rated maximum injected apparent power on phase L3, defined by min(EV, EVSE) in va.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_apparent_power_l3: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_reactive_power: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars on phase L2.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_reactive_power_l2: Option<f64>, // decimal

    /// Optional. Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars on phase L3.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charge_reactive_power_l3: Option<f64>, // decimal

    /// Optional. Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the AvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_reactive_power: Option<f64>, // decimal

    /// Optional. Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars on phase L2.
    /// Corresponds to the AvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_reactive_power_l2: Option<f64>, // decimal

    /// Optional. Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars on phase L3.
    /// Corresponds to the AvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charge_reactive_power_l3: Option<f64>, // decimal

    /// Optional. Rated maximum injected reactive power, defined by min(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_reactive_power: Option<f64>, // decimal

    /// Optional. Rated maximum injected reactive power, defined by min(EV, EVSE), in vars on phase L2.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_reactive_power_l2: Option<f64>, // decimal

    /// Optional. Rated maximum injected reactive power, defined by min(EV, EVSE), in vars on phase L3.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discharge_reactive_power_l3: Option<f64>, // decimal

    /// Optional. Rated minimum injected reactive power, defined by max(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the IvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_reactive_power: Option<f64>, // decimal

    /// Optional. Rated minimum injected reactive power, defined by max(EV, EVSE), in vars on phase L2.
    /// Corresponds to the IvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower_L2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_reactive_power_l2: Option<f64>, // decimal

    /// Optional. Rated minimum injected reactive power, defined by max(EV, EVSE), in vars on phase L3.
    /// Corresponds to the IvarMin attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower_L3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_discharge_reactive_power_l3: Option<f64>, // decimal

    /// Optional. Line voltage supported by EVSE and EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVNominalVoltage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_voltage: Option<f64>, // decimal

    /// Optional. The nominal AC voltage (rms) offset between the Charging Station's electrical connection point and the utility's point of common coupling.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVNominalVoltageOffset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_voltage_offset: Option<f64>, // decimal

    /// Optional. Optional maximum AC rms voltage, as defined by min(EV, EVSE) to operate with.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumNominalVoltage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nominal_voltage: Option<f64>, // decimal

    /// Optional. Optional minimum AC rms voltage, as defined by max(EV, EVSE) to operate with.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumNominalVoltage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nominal_voltage: Option<f64>, // decimal

    /// Optional. Manufacturer of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterManufacturer
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_inverter_manufacturer: Option<String>,

    /// Optional. Model name of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterModel
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_inverter_model: Option<String>,

    /// Optional. Serial number of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterSerialNumber
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_inverter_serial_number: Option<String>,

    /// Optional. Software version of EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterSwVersion
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_inverter_sw_version: Option<String>,

    /// Optional. Hardware version of EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterHwVersion
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_inverter_hw_version: Option<String>,

    /// Optional. Type of islanding detection method.
    /// Only mandatory when islanding detection is required at the site.
    /// Is set in the ISO 15118 Service Details configuration.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVIslandingDetectionMethod
    /// Cardinality 0..*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_islanding_detection_method: Option<Vec<IslandingDetectionEnumType>>,

    /// Optional. Time after which EV will trip if an island has been detected.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVIslandingTripTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_islanding_trip_time: Option<f64>, // decimal

    /// Optional. Maximum injected DC current allowed at level 1 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumLevel1DCInjection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_maximum_level1_dc_injection: Option<f64>, // decimal

    /// Optional. Maximum allowed duration of DC injection at level 1 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVDurationLevel1DCInjection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_duration_level1_dc_injection: Option<f64>, // decimal

    /// Optional. Maximum injected DC current allowed at level 2 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumLevel2DCInjection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_maximum_level2_dc_injection: Option<f64>, // decimal

    /// Optional. Maximum allowed duration of DC injection at level 2 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVDurationLevel2DCInjection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_duration_level2_dc_injection: Option<f64>, // decimal

    /// Optional. Measure of the susceptibility of the circuit to reactance, in Siemens (S).
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVReactiveSusceptance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_reactive_susceptance: Option<f64>, // decimal

    /// Optional. Total energy value, in Wh, that EV is allowed to provide during the entire V2G session.
    /// The value is independent of the V2X Cycling area. Once this value reaches the value of 0,
    /// the EV may block any attempt to discharge in order to protect the battery health.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVSessionTotalDischargeEnergyAvailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_session_total_discharge_energy_available: Option<f64>, // decimal
}

impl OcppEntity for DERChargingParametersType {
    /// Validates the fields of DERChargingParametersType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        if let Some(ev_inverter_manufacturer) = &self.ev_inverter_manufacturer {
            e.check_cardinality(
                "ev_inverter_manufacturer",
                0,
                50,
                &ev_inverter_manufacturer.chars(),
            );
        }

        if let Some(ev_inverter_model) = &self.ev_inverter_model {
            e.check_cardinality("ev_inverter_model", 0, 50, &ev_inverter_model.chars());
        }

        if let Some(ev_inverter_serial_number) = &self.ev_inverter_serial_number {
            e.check_cardinality(
                "ev_inverter_serial_number",
                0,
                50,
                &ev_inverter_serial_number.chars(),
            );
        }

        if let Some(ev_inverter_sw_version) = &self.ev_inverter_sw_version {
            e.check_cardinality(
                "ev_inverter_sw_version",
                0,
                50,
                &ev_inverter_sw_version.chars(),
            );
        }

        if let Some(ev_inverter_hw_version) = &self.ev_inverter_hw_version {
            e.check_cardinality(
                "ev_inverter_hw_version",
                0,
                50,
                &ev_inverter_hw_version.chars(),
            );
        }

        e.build("DERChargingParametersType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_serialization_deserialization() {
        let der_params = DERChargingParametersType {
            ev_supported_der_control: Some(vec![DERControlEnumType::FreqWatt]),
            ev_over_excited_max_discharge_power: Some(100.0),
            ev_over_excited_power_factor: Some(0.9),
            ev_under_excited_max_discharge_power: Some(80.0),
            ev_under_excited_power_factor: Some(0.8),
            max_apparent_power: Some(200.0),
            max_charge_apparent_power: Some(150.0),
            max_charge_apparent_power_l2: Some(75.0),
            max_charge_apparent_power_l3: Some(75.0),
            max_discharge_apparent_power: Some(120.0),
            max_discharge_apparent_power_l2: Some(60.0),
            max_discharge_apparent_power_l3: Some(60.0),
            max_charge_reactive_power: Some(50.0),
            max_charge_reactive_power_l2: Some(25.0),
            max_charge_reactive_power_l3: Some(25.0),
            min_charge_reactive_power: Some(-10.0),
            min_charge_reactive_power_l2: Some(-5.0),
            min_charge_reactive_power_l3: Some(-5.0),
            max_discharge_reactive_power: Some(40.0),
            max_discharge_reactive_power_l2: Some(20.0),
            max_discharge_reactive_power_l3: Some(20.0),
            min_discharge_reactive_power: Some(-8.0),
            min_discharge_reactive_power_l2: Some(-4.0),
            min_discharge_reactive_power_l3: Some(-4.0),
            nominal_voltage: Some(230.0),
            nominal_voltage_offset: Some(5.0),
            max_nominal_voltage: Some(250.0),
            min_nominal_voltage: Some(210.0),
            ev_inverter_manufacturer: Some("InvManuf".to_string()),
            ev_inverter_model: Some("InvModelX".to_string()),
            ev_inverter_serial_number: Some("INV-SN-123".to_string()),
            ev_inverter_sw_version: Some("1.0.0".to_string()),
            ev_inverter_hw_version: Some("A.B".to_string()),
            ev_islanding_detection_method: Some(vec![IslandingDetectionEnumType::RoCoF]),
            ev_islanding_trip_time: Some(0.5),
            ev_maximum_level1_dc_injection: Some(10.0),
            ev_duration_level1_dc_injection: Some(300.0),
            ev_maximum_level2_dc_injection: Some(20.0),
            ev_duration_level2_dc_injection: Some(600.0),
            ev_reactive_susceptance: Some(0.01),
            ev_session_total_discharge_energy_available: Some(5000.0),
        };

        let serialized = serde_json::to_string(&der_params).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: DERChargingParametersType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(der_params, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let der_params_minimal = DERChargingParametersType {
            ev_supported_der_control: None,
            ev_over_excited_max_discharge_power: None,
            ev_over_excited_power_factor: None,
            ev_under_excited_max_discharge_power: None,
            ev_under_excited_power_factor: None,
            max_apparent_power: None,
            max_charge_apparent_power: None,
            max_charge_apparent_power_l2: None,
            max_charge_apparent_power_l3: None,
            max_discharge_apparent_power: None,
            max_discharge_apparent_power_l2: None,
            max_discharge_apparent_power_l3: None,
            max_charge_reactive_power: None,
            max_charge_reactive_power_l2: None,
            max_charge_reactive_power_l3: None,
            min_charge_reactive_power: None,
            min_charge_reactive_power_l2: None,
            min_charge_reactive_power_l3: None,
            max_discharge_reactive_power: None,
            max_discharge_reactive_power_l2: None,
            max_discharge_reactive_power_l3: None,
            min_discharge_reactive_power: None,
            min_discharge_reactive_power_l2: None,
            min_discharge_reactive_power_l3: None,
            nominal_voltage: None,
            nominal_voltage_offset: None,
            max_nominal_voltage: None,
            min_nominal_voltage: None,
            ev_inverter_manufacturer: None,
            ev_inverter_model: None,
            ev_inverter_serial_number: None,
            ev_inverter_sw_version: None,
            ev_inverter_hw_version: None,
            ev_islanding_detection_method: None,
            ev_islanding_trip_time: None,
            ev_maximum_level1_dc_injection: None,
            ev_duration_level1_dc_injection: None,
            ev_maximum_level2_dc_injection: None,
            ev_duration_level2_dc_injection: None,
            ev_reactive_susceptance: None,
            ev_session_total_discharge_energy_available: None,
        };
        assert!(der_params_minimal.validate().is_ok());

        let der_params_max_lengths = DERChargingParametersType {
            ev_supported_der_control: None, // No length constraint on Vec for now
            ev_over_excited_max_discharge_power: None,
            ev_over_excited_power_factor: None,
            ev_under_excited_max_discharge_power: None,
            ev_under_excited_power_factor: None,
            max_apparent_power: None,
            max_charge_apparent_power: None,
            max_charge_apparent_power_l2: None,
            max_charge_apparent_power_l3: None,
            max_discharge_apparent_power: None,
            max_discharge_apparent_power_l2: None,
            max_discharge_apparent_power_l3: None,
            max_charge_reactive_power: None,
            max_charge_reactive_power_l2: None,
            max_charge_reactive_power_l3: None,
            min_charge_reactive_power: None,
            min_charge_reactive_power_l2: None,
            min_charge_reactive_power_l3: None,
            max_discharge_reactive_power: None,
            max_discharge_reactive_power_l2: None,
            max_discharge_reactive_power_l3: None,
            min_discharge_reactive_power: None,
            min_discharge_reactive_power_l2: None,
            min_discharge_reactive_power_l3: None,
            nominal_voltage: None,
            nominal_voltage_offset: None,
            max_nominal_voltage: None,
            min_nominal_voltage: None,
            ev_inverter_manufacturer: Some("a".repeat(50)),
            ev_inverter_model: Some("b".repeat(50)),
            ev_inverter_serial_number: Some("c".repeat(50)),
            ev_inverter_sw_version: Some("d".repeat(50)),
            ev_inverter_hw_version: Some("e".repeat(50)),
            ev_islanding_detection_method: None, // No length constraint on Vec for now
            ev_islanding_trip_time: None,
            ev_maximum_level1_dc_injection: None,
            ev_duration_level1_dc_injection: None,
            ev_maximum_level2_dc_injection: None,
            ev_duration_level2_dc_injection: None,
            ev_reactive_susceptance: None,
            ev_session_total_discharge_energy_available: None,
        };
        assert!(der_params_max_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_ev_inverter_manufacturer_too_long() {
        let der_params = DERChargingParametersType {
            ev_inverter_manufacturer: Some("a".repeat(51)), // Invalid
            ..Default::default()                            // Use default for other fields
        };
        let err = der_params.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_inverter_manufacturer");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_string_errors() {
        let der_params = DERChargingParametersType {
            ev_inverter_manufacturer: Some("a".repeat(51)), // Invalid 1
            ev_inverter_model: Some("b".repeat(51)),        // Invalid 2
            ev_inverter_serial_number: Some("c".repeat(51)), // Invalid 3
            ev_inverter_sw_version: Some("d".repeat(51)),   // Invalid 4
            ev_inverter_hw_version: Some("e".repeat(51)),   // Invalid 5
            ..Default::default()
        };
        let err = der_params.validate().unwrap_err();
        assert_invalid_fields(
            &err,
            &[
                "ev_inverter_manufacturer",
                "ev_inverter_model",
                "ev_inverter_serial_number",
                "ev_inverter_sw_version",
                "ev_inverter_hw_version",
            ],
        );
    }
}

// Implement Default trait for easier testing with partial data
impl Default for DERChargingParametersType {
    fn default() -> Self {
        DERChargingParametersType {
            ev_supported_der_control: None,
            ev_over_excited_max_discharge_power: None,
            ev_over_excited_power_factor: None,
            ev_under_excited_max_discharge_power: None,
            ev_under_excited_power_factor: None,
            max_apparent_power: None,
            max_charge_apparent_power: None,
            max_charge_apparent_power_l2: None,
            max_charge_apparent_power_l3: None,
            max_discharge_apparent_power: None,
            max_discharge_apparent_power_l2: None,
            max_discharge_apparent_power_l3: None,
            max_charge_reactive_power: None,
            max_charge_reactive_power_l2: None,
            max_charge_reactive_power_l3: None,
            min_charge_reactive_power: None,
            min_charge_reactive_power_l2: None,
            min_charge_reactive_power_l3: None,
            max_discharge_reactive_power: None,
            max_discharge_reactive_power_l2: None,
            max_discharge_reactive_power_l3: None,
            min_discharge_reactive_power: None,
            min_discharge_reactive_power_l2: None,
            min_discharge_reactive_power_l3: None,
            nominal_voltage: None,
            nominal_voltage_offset: None,
            max_nominal_voltage: None,
            min_nominal_voltage: None,
            ev_inverter_manufacturer: None,
            ev_inverter_model: None,
            ev_inverter_serial_number: None,
            ev_inverter_sw_version: None,
            ev_inverter_hw_version: None,
            ev_islanding_detection_method: None,
            ev_islanding_trip_time: None,
            ev_maximum_level1_dc_injection: None,
            ev_duration_level1_dc_injection: None,
            ev_maximum_level2_dc_injection: None,
            ev_duration_level2_dc_injection: None,
            ev_reactive_susceptance: None,
            ev_session_total_discharge_energy_available: None,
        }
    }
}
