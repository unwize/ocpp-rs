use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Reason that triggered a transactionEventRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerReasonEnumType {
    /// An Abnormal Error or Fault Condition has occurred.
    AbnormalCondition,
    /// Charging is authorized, by any means. Might be an RFID, or other authorization means.
    Authorized,
    /// Cable is plugged in and EVDetected.
    CablePluggedIn,
    /// Rate of charging changed by more than `LimitChangeSignificance` by an external actor (e.g. an EMS).
    ChargingRateChanged,
    /// Charging State changed.
    ChargingStateChanged,
    /// (2.1) Maximum cost has been reached, as defined by `transactionLimit.maxCost`.
    CostLimitReached,
    /// The transaction was stopped because of the authorization status in the response to a transactionEventRequest.
    Deauthorized,
    /// Maximum energy of charging reached as defined by `transactionLimit.maxEnergy`.
    EnergyLimitReached,
    /// Communication with EV lost, for example: cable disconnected.
    EVCommunicationLoss,
    /// EV not connected before the connection is timed out.
    EVConnectTimeout,
    /// EV departed. For example: When a departing EV triggers a parking bay detector.
    EVDeparted,
    /// EV detected. For example: When an arriving EV triggers a parking bay detector.
    EVDetected,
    /// (2.1) Limit of cost/time/energy/SoC for transaction has set or changed.
    LimitSet,
    /// Needed to send a clock aligned meter value.
    MeterValueClock,
    /// Needed to send a periodic meter value.
    MeterValuePeriodic,
    /// (2.1) V2X operation mode has changed (at start of a new charging schedule period).
    OperationModeChanged,
    /// A `RequestStartTransactionRequest` has been sent.
    RemoteStart,
    /// A `RequestStopTransactionRequest` has been sent.
    RemoteStop,
    /// CSMS sent a Reset Charging Station command.
    ResetCommand,
    /// (2.1) Trigger used when TransactionEvent is sent (only) to report a running cost update.
    RunningCost,
    /// Signed data is received from the energy meter.
    SignedDataReceived,
    /// (2.1) State of charge limit has been reached, as defined by `transactionLimit.maxSoc`.
    SOCLimitReached,
    /// An EV Driver has been authorized to stop charging. For example: By swiping an RFID card.
    StopAuthorized,
    /// (2.1) Tariff for transaction has changed.
    TariffNotAccepted,
    /// (2.1) Trigger to notify that EV Driver has not accepted the tariff for transaction. IdToken becomes unauthorized.
    TariffChange,
    /// (2.1) Maximum time of charging reached, as defined by `transactionLimit.maxTime`.
    TimeLimitReached,
    /// Requested by the CSMS via a `TriggerMessageRequest`.
    Trigger,
    /// (2.1) Transaction has resumed after reset or power outage.
    TxResumed,
    /// CSMS sent an Unlock Connector command.
    UnlockCommand,
}

impl fmt::Display for TriggerReasonEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AbnormalCondition => write!(f, "AbnormalCondition"),
            Self::Authorized => write!(f, "Authorized"),
            Self::CablePluggedIn => write!(f, "CablePluggedIn"),
            Self::ChargingRateChanged => write!(f, "ChargingRateChanged"),
            Self::ChargingStateChanged => write!(f, "ChargingStateChanged"),
            Self::CostLimitReached => write!(f, "CostLimitReached"),
            Self::Deauthorized => write!(f, "Deauthorized"),
            Self::EnergyLimitReached => write!(f, "EnergyLimitReached"),
            Self::EVCommunicationLoss => write!(f, "EVCommunicationLoss"),
            Self::EVConnectTimeout => write!(f, "EVConnectTimeout"),
            Self::EVDeparted => write!(f, "EVDeparted"),
            Self::EVDetected => write!(f, "EVDetected"),
            Self::LimitSet => write!(f, "LimitSet"),
            Self::MeterValueClock => write!(f, "MeterValueClock"),
            Self::MeterValuePeriodic => write!(f, "MeterValuePeriodic"),
            Self::OperationModeChanged => write!(f, "OperationModeChanged"),
            Self::RemoteStart => write!(f, "RemoteStart"),
            Self::RemoteStop => write!(f, "RemoteStop"),
            Self::ResetCommand => write!(f, "ResetCommand"),
            Self::RunningCost => write!(f, "RunningCost"),
            Self::SignedDataReceived => write!(f, "SignedDataReceived"),
            Self::SOCLimitReached => write!(f, "SOCLimitReached"),
            Self::StopAuthorized => write!(f, "StopAuthorized"),
            Self::TariffNotAccepted => write!(f, "TariffNotAccepted"),
            Self::TariffChange => write!(f, "TariffChange"),
            Self::TimeLimitReached => write!(f, "TimeLimitReached"),
            Self::Trigger => write!(f, "Trigger"),
            Self::TxResumed => write!(f, "TxResumed"),
            Self::UnlockCommand => write!(f, "UnlockCommand"),
        }
    }
}

impl Into<String> for TriggerReasonEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TriggerReasonEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AbnormalCondition" => Ok(Self::AbnormalCondition),
            "Authorized" => Ok(Self::Authorized),
            "CablePluggedIn" => Ok(Self::CablePluggedIn),
            "ChargingRateChanged" => Ok(Self::ChargingRateChanged),
            "ChargingStateChanged" => Ok(Self::ChargingStateChanged),
            "CostLimitReached" => Ok(Self::CostLimitReached),
            "Deauthorized" => Ok(Self::Deauthorized),
            "EnergyLimitReached" => Ok(Self::EnergyLimitReached),
            "EVCommunicationLoss" => Ok(Self::EVCommunicationLoss),
            "EVConnectTimeout" => Ok(Self::EVConnectTimeout),
            "EVDeparted" => Ok(Self::EVDeparted),
            "EVDetected" => Ok(Self::EVDetected),
            "LimitSet" => Ok(Self::LimitSet),
            "MeterValueClock" => Ok(Self::MeterValueClock),
            "MeterValuePeriodic" => Ok(Self::MeterValuePeriodic),
            "OperationModeChanged" => Ok(Self::OperationModeChanged),
            "RemoteStart" => Ok(Self::RemoteStart),
            "RemoteStop" => Ok(Self::RemoteStop),
            "ResetCommand" => Ok(Self::ResetCommand),
            "RunningCost" => Ok(Self::RunningCost),
            "SignedDataReceived" => Ok(Self::SignedDataReceived),
            "SOCLimitReached" => Ok(Self::SOCLimitReached),
            "StopAuthorized" => Ok(Self::StopAuthorized),
            "TariffNotAccepted" => Ok(Self::TariffNotAccepted),
            "TariffChange" => Ok(Self::TariffChange),
            "TimeLimitReached" => Ok(Self::TimeLimitReached),
            "Trigger" => Ok(Self::Trigger),
            "TxResumed" => Ok(Self::TxResumed),
            "UnlockCommand" => Ok(Self::UnlockCommand),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TriggerReasonEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
