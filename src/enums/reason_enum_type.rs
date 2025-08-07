use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Reason for stopping a transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReasonEnumType {
    /// The transaction was stopped because of the authorization status in the response to a TransactionEventRequest. (Failed)
    DeAuthorized,
    /// Emergency stop button was used. (Failed)
    EmergencyStop,
    /// (2.1) Deprecated, because it stops energy transfer, not the transaction. EV charging session reached a locally enforced maximum energy transfer limit. (Successful)
    EnergyLimitReached,
    /// Disconnecting of cable, vehicle moved away from inductive charge unit. (Successful)
    EVDisconnected,
    /// A GroundFault has occurred. (Failed)
    GroundFault,
    /// A Reset(Immediate) command was received. (Failed)
    ImmediateReset,
    /// The transaction was stopped using a token that belongs to the MasterPassGroupId. (Successful)
    MasterPass,
    /// Stopped locally on request of the EV Driver at the Charge Point. This is a regular termination of a transaction. Examples: presenting an IdToken tag, pressing a button to stop. (Successful)
    Local,
    /// (2.1) Deprecated, because it stops energy transfer, not the transaction. A local credit limit enforced through the Charging Station has been exceeded. (Successful)
    LocalOutOfCredit,
    /// Any other reason. (Failed)
    Other,
    /// A larger than intended electric current has occurred. (Failed)
    OvercurrentFault,
    /// Complete loss of power. (Failed)
    PowerLoss,
    /// Quality of power too low, e.g. voltage too low/high, phase imbalance, etc. (Failed)
    PowerQuality,
    /// A locally initiated reset/reboot occurred. (for instance watchdog kicked in). (Failed)
    Reboot,
    /// Stopped remotely on request of the CSMS. This is a regular termination of a transaction. Examples: termination using a smartphone app, exceeding a (non local) prepaid credit. (Successful)
    Remote,
    /// (2.1) Deprecated, because it stops energy transfer, not the transaction. Electric vehicle has reported a reaching locally enforced maximum battery State of Charge (SOC). (Successful)
    SOCLimitReached,
    /// The transaction was stopped by the EV. (Successful)
    StoppedByEV,
    /// (2.1) Deprecated, because it stops energy transfer, not the transaction. EV charging session reached a locally enforced time limit. (Successful)
    TimeLimitReached,
    /// EV not connected within timeout. (Failed)
    Timeout,
    /// (2.1) CSMS cannot accept the requested energy transfer type. (Failed)
    ReqEnergyTransferRejected,
}

impl fmt::Display for ReasonEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DeAuthorized => write!(f, "DeAuthorized"),
            Self::EmergencyStop => write!(f, "EmergencyStop"),
            Self::EnergyLimitReached => write!(f, "EnergyLimitReached"),
            Self::EVDisconnected => write!(f, "EVDisconnected"),
            Self::GroundFault => write!(f, "GroundFault"),
            Self::ImmediateReset => write!(f, "ImmediateReset"),
            Self::MasterPass => write!(f, "MasterPass"),
            Self::Local => write!(f, "Local"),
            Self::LocalOutOfCredit => write!(f, "LocalOutOfCredit"),
            Self::Other => write!(f, "Other"),
            Self::OvercurrentFault => write!(f, "OvercurrentFault"),
            Self::PowerLoss => write!(f, "PowerLoss"),
            Self::PowerQuality => write!(f, "PowerQuality"),
            Self::Reboot => write!(f, "Reboot"),
            Self::Remote => write!(f, "Remote"),
            Self::SOCLimitReached => write!(f, "SOCLimitReached"),
            Self::StoppedByEV => write!(f, "StoppedByEV"),
            Self::TimeLimitReached => write!(f, "TimeLimitReached"),
            Self::Timeout => write!(f, "Timeout"),
            Self::ReqEnergyTransferRejected => write!(f, "ReqEnergyTransferRejected"),
        }
    }
}

impl Into<String> for ReasonEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ReasonEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DeAuthorized" => Ok(Self::DeAuthorized),
            "EmergencyStop" => Ok(Self::EmergencyStop),
            "EnergyLimitReached" => Ok(Self::EnergyLimitReached),
            "EVDisconnected" => Ok(Self::EVDisconnected),
            "GroundFault" => Ok(Self::GroundFault),
            "ImmediateReset" => Ok(Self::ImmediateReset),
            "MasterPass" => Ok(Self::MasterPass),
            "Local" => Ok(Self::Local),
            "LocalOutOfCredit" => Ok(Self::LocalOutOfCredit),
            "Other" => Ok(Self::Other),
            "OvercurrentFault" => Ok(Self::OvercurrentFault),
            "PowerLoss" => Ok(Self::PowerLoss),
            "PowerQuality" => Ok(Self::PowerQuality),
            "Reboot" => Ok(Self::Reboot),
            "Remote" => Ok(Self::Remote),
            "SOCLimitReached" => Ok(Self::SOCLimitReached),
            "StoppedByEV" => Ok(Self::StoppedByEV),
            "TimeLimitReached" => Ok(Self::TimeLimitReached),
            "Timeout" => Ok(Self::Timeout),
            "ReqEnergyTransferRejected" => Ok(Self::ReqEnergyTransferRejected),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ReasonEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}