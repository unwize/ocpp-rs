use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Allowable values of the optional "measurand" field of a Value element, as used in MeterValuesRequest and TransactionEventRequest.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MeasurandEnumType {
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Export.Offered")]
    CurrentExportOffered,
    #[serde(rename = "Current.Export.Minimum")]
    CurrentExportMinimum,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Import.Offered")]
    CurrentImportOffered,
    #[serde(rename = "Current.Import.Minimum")]
    CurrentImportMinimum,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Display.PresentSOC")]
    DisplayPresentSoc,
    #[serde(rename = "Display.MinimumSOC")]
    DisplayMinimumSoc,
    #[serde(rename = "Display.TargetSOC")]
    DisplayTargetSoc,
    #[serde(rename = "Display.MaximumSOC")]
    DisplayMaximumSoc,
    #[serde(rename = "Display.RemainingTimeToMinimumSOC")]
    DisplayRemainingTimeToMinimumSoc,
    #[serde(rename = "Display.RemainingTimeToTargetSOC")]
    DisplayRemainingTimeToTargetSoc,
    #[serde(rename = "Display.RemainingTimeToMaximumSOC")]
    DisplayRemainingTimeToMaximumSoc,
    #[serde(rename = "Display.ChargingComplete")]
    DisplayChargingComplete,
    #[serde(rename = "Display.BatteryEnergyCapacity")]
    DisplayBatteryEnergyCapacity,
    #[serde(rename = "Display.InletHot")]
    DisplayInletHot,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Active.Import.CableLoss")]
    EnergyActiveImportCableLoss,
    #[serde(rename = "Energy.Active.Import.LocalGeneration.Register")]
    EnergyActiveImportLocalGenerationRegister,
    #[serde(rename = "Energy.Active.Net")]
    EnergyActiveNet,
    #[serde(rename = "Energy.Active.Setpoint.Interval")]
    EnergyActiveSetpointInterval,
    #[serde(rename = "Energy.Apparent.Export")]
    EnergyApparentExport,
    #[serde(rename = "Energy.Apparent.Import")]
    EnergyApparentImport,
    #[serde(rename = "Energy.Apparent.Net")]
    EnergyApparentNet,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Reactive.Net")]
    EnergyReactiveNet,
    #[serde(rename = "EnergyRequest.Target")]
    EnergyRequestTarget,
    #[serde(rename = "EnergyRequest.Minimum")]
    EnergyRequestMinimum,
    #[serde(rename = "EnergyRequest.Maximum")]
    EnergyRequestMaximum,
    #[serde(rename = "EnergyRequest.Minimum.V2X")]
    EnergyRequestMinimumV2x,
    #[serde(rename = "EnergyRequest.Maximum.V2X")]
    EnergyRequestMaximumV2x,
    #[serde(rename = "EnergyRequest.Bulk")]
    EnergyRequestBulk,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Active.Setpoint")]
    PowerActiveSetpoint,
    #[serde(rename = "Power.Active.Residual")]
    PowerActiveResidual,
    #[serde(rename = "Power.Export.Minimum")]
    PowerExportMinimum,
    #[serde(rename = "Power.Export.Offered")]
    PowerExportOffered,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Power.Import.Offered")]
    PowerImportOffered,
    #[serde(rename = "Power.Import.Minimum")]
    PowerImportMinimum,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "Voltage")]
    Voltage,
    #[serde(rename = "Voltage.Minimum")]
    VoltageMinimum,
    #[serde(rename = "Voltage.Maximum")]
    VoltageMaximum,
}

impl fmt::Display for MeasurandEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CurrentExport => write!(f, "Current.Export"),
            Self::CurrentExportOffered => write!(f, "Current.Export.Offered"),
            Self::CurrentExportMinimum => write!(f, "Current.Export.Minimum"),
            Self::CurrentImport => write!(f, "Current.Import"),
            Self::CurrentImportOffered => write!(f, "Current.Import.Offered"),
            Self::CurrentImportMinimum => write!(f, "Current.Import.Minimum"),
            Self::CurrentOffered => write!(f, "Current.Offered"),
            Self::DisplayPresentSoc => write!(f, "Display.PresentSOC"),
            Self::DisplayMinimumSoc => write!(f, "Display.MinimumSOC"),
            Self::DisplayTargetSoc => write!(f, "Display.TargetSOC"),
            Self::DisplayMaximumSoc => write!(f, "Display.MaximumSOC"),
            Self::DisplayRemainingTimeToMinimumSoc => write!(f, "Display.RemainingTimeToMinimumSOC"),
            Self::DisplayRemainingTimeToTargetSoc => write!(f, "Display.RemainingTimeToTargetSOC"),
            Self::DisplayRemainingTimeToMaximumSoc => write!(f, "Display.RemainingTimeToMaximumSOC"),
            Self::DisplayChargingComplete => write!(f, "Display.ChargingComplete"),
            Self::DisplayBatteryEnergyCapacity => write!(f, "Display.BatteryEnergyCapacity"),
            Self::DisplayInletHot => write!(f, "Display.InletHot"),
            Self::EnergyActiveExportInterval => write!(f, "Energy.Active.Export.Interval"),
            Self::EnergyActiveExportRegister => write!(f, "Energy.Active.Export.Register"),
            Self::EnergyActiveImportInterval => write!(f, "Energy.Active.Import.Interval"),
            Self::EnergyActiveImportRegister => write!(f, "Energy.Active.Import.Register"),
            Self::EnergyActiveImportCableLoss => write!(f, "Energy.Active.Import.CableLoss"),
            Self::EnergyActiveImportLocalGenerationRegister => write!(f, "Energy.Active.Import.LocalGeneration.Register"),
            Self::EnergyActiveNet => write!(f, "Energy.Active.Net"),
            Self::EnergyActiveSetpointInterval => write!(f, "Energy.Active.Setpoint.Interval"),
            Self::EnergyApparentExport => write!(f, "Energy.Apparent.Export"),
            Self::EnergyApparentImport => write!(f, "Energy.Apparent.Import"),
            Self::EnergyApparentNet => write!(f, "Energy.Apparent.Net"),
            Self::EnergyReactiveExportInterval => write!(f, "Energy.Reactive.Export.Interval"),
            Self::EnergyReactiveExportRegister => write!(f, "Energy.Reactive.Export.Register"),
            Self::EnergyReactiveImportInterval => write!(f, "Energy.Reactive.Import.Interval"),
            Self::EnergyReactiveImportRegister => write!(f, "Energy.Reactive.Import.Register"),
            Self::EnergyReactiveNet => write!(f, "Energy.Reactive.Net"),
            Self::EnergyRequestTarget => write!(f, "EnergyRequest.Target"),
            Self::EnergyRequestMinimum => write!(f, "EnergyRequest.Minimum"),
            Self::EnergyRequestMaximum => write!(f, "EnergyRequest.Maximum"),
            Self::EnergyRequestMinimumV2x => write!(f, "EnergyRequest.Minimum.V2X"),
            Self::EnergyRequestMaximumV2x => write!(f, "EnergyRequest.Maximum.V2X"),
            Self::EnergyRequestBulk => write!(f, "EnergyRequest.Bulk"),
            Self::Frequency => write!(f, "Frequency"),
            Self::PowerActiveExport => write!(f, "Power.Active.Export"),
            Self::PowerActiveImport => write!(f, "Power.Active.Import"),
            Self::PowerActiveSetpoint => write!(f, "Power.Active.Setpoint"),
            Self::PowerActiveResidual => write!(f, "Power.Active.Residual"),
            Self::PowerExportMinimum => write!(f, "Power.Export.Minimum"),
            Self::PowerExportOffered => write!(f, "Power.Export.Offered"),
            Self::PowerFactor => write!(f, "Power.Factor"),
            Self::PowerImportOffered => write!(f, "Power.Import.Offered"),
            Self::PowerImportMinimum => write!(f, "Power.Import.Minimum"),
            Self::PowerOffered => write!(f, "Power.Offered"),
            Self::PowerReactiveExport => write!(f, "Power.Reactive.Export"),
            Self::PowerReactiveImport => write!(f, "Power.Reactive.Import"),
            Self::SoC => write!(f, "SoC"),
            Self::Voltage => write!(f, "Voltage"),
            Self::VoltageMinimum => write!(f, "Voltage.Minimum"),
            Self::VoltageMaximum => write!(f, "Voltage.Maximum"),
        }
    }
}

impl Into<String> for MeasurandEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MeasurandEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Current.Export" => Ok(Self::CurrentExport),
            "Current.Export.Offered" => Ok(Self::CurrentExportOffered),
            "Current.Export.Minimum" => Ok(Self::CurrentExportMinimum),
            "Current.Import" => Ok(Self::CurrentImport),
            "Current.Import.Offered" => Ok(Self::CurrentImportOffered),
            "Current.Import.Minimum" => Ok(Self::CurrentImportMinimum),
            "Current.Offered" => Ok(Self::CurrentOffered),
            "Display.PresentSOC" => Ok(Self::DisplayPresentSoc),
            "Display.MinimumSOC" => Ok(Self::DisplayMinimumSoc),
            "Display.TargetSOC" => Ok(Self::DisplayTargetSoc),
            "Display.MaximumSOC" => Ok(Self::DisplayMaximumSoc),
            "Display.RemainingTimeToMinimumSOC" => Ok(Self::DisplayRemainingTimeToMinimumSoc),
            "Display.RemainingTimeToTargetSOC" => Ok(Self::DisplayRemainingTimeToTargetSoc),
            "Display.RemainingTimeToMaximumSOC" => Ok(Self::DisplayRemainingTimeToMaximumSoc),
            "Display.ChargingComplete" => Ok(Self::DisplayChargingComplete),
            "Display.BatteryEnergyCapacity" => Ok(Self::DisplayBatteryEnergyCapacity),
            "Display.InletHot" => Ok(Self::DisplayInletHot),
            "Energy.Active.Export.Interval" => Ok(Self::EnergyActiveExportInterval),
            "Energy.Active.Export.Register" => Ok(Self::EnergyActiveExportRegister),
            "Energy.Active.Import.Interval" => Ok(Self::EnergyActiveImportInterval),
            "Energy.Active.Import.Register" => Ok(Self::EnergyActiveImportRegister),
            "Energy.Active.Import.CableLoss" => Ok(Self::EnergyActiveImportCableLoss),
            "Energy.Active.Import.LocalGeneration.Register" => Ok(Self::EnergyActiveImportLocalGenerationRegister),
            "Energy.Active.Net" => Ok(Self::EnergyActiveNet),
            "Energy.Active.Setpoint.Interval" => Ok(Self::EnergyActiveSetpointInterval),
            "Energy.Apparent.Export" => Ok(Self::EnergyApparentExport),
            "Energy.Apparent.Import" => Ok(Self::EnergyApparentImport),
            "Energy.Apparent.Net" => Ok(Self::EnergyApparentNet),
            "Energy.Reactive.Export.Interval" => Ok(Self::EnergyReactiveExportInterval),
            "Energy.Reactive.Export.Register" => Ok(Self::EnergyReactiveExportRegister),
            "Energy.Reactive.Import.Interval" => Ok(Self::EnergyReactiveImportInterval),
            "Energy.Reactive.Import.Register" => Ok(Self::EnergyReactiveImportRegister),
            "Energy.Reactive.Net" => Ok(Self::EnergyReactiveNet),
            "EnergyRequest.Target" => Ok(Self::EnergyRequestTarget),
            "EnergyRequest.Minimum" => Ok(Self::EnergyRequestMinimum),
            "EnergyRequest.Maximum" => Ok(Self::EnergyRequestMaximum),
            "EnergyRequest.Minimum.V2X" => Ok(Self::EnergyRequestMinimumV2x),
            "EnergyRequest.Maximum.V2X" => Ok(Self::EnergyRequestMaximumV2x),
            "EnergyRequest.Bulk" => Ok(Self::EnergyRequestBulk),
            "Frequency" => Ok(Self::Frequency),
            "Power.Active.Export" => Ok(Self::PowerActiveExport),
            "Power.Active.Import" => Ok(Self::PowerActiveImport),
            "Power.Active.Setpoint" => Ok(Self::PowerActiveSetpoint),
            "Power.Active.Residual" => Ok(Self::PowerActiveResidual),
            "Power.Export.Minimum" => Ok(Self::PowerExportMinimum),
            "Power.Export.Offered" => Ok(Self::PowerExportOffered),
            "Power.Factor" => Ok(Self::PowerFactor),
            "Power.Import.Offered" => Ok(Self::PowerImportOffered),
            "Power.Import.Minimum" => Ok(Self::PowerImportMinimum),
            "Power.Offered" => Ok(Self::PowerOffered),
            "Power.Reactive.Export" => Ok(Self::PowerReactiveExport),
            "Power.Reactive.Import" => Ok(Self::PowerReactiveImport),
            "SoC" => Ok(Self::SoC),
            "Voltage" => Ok(Self::Voltage),
            "Voltage.Minimum" => Ok(Self::VoltageMinimum),
            "Voltage.Maximum" => Ok(Self::VoltageMaximum),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MeasurandEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}