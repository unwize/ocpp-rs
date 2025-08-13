use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Type of request to be triggered by trigger messages.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageTriggerEnumType {
    /// To trigger BootNotification.
    BootNotification,
    /// To trigger LogStatusNotification.
    LogStatusNotification,
    /// To trigger FirmwareStatusNotification.
    FirmwareStatusNotification,
    /// To trigger Heartbeat.
    Heartbeat,
    /// To trigger MeterValues.
    MeterValues,
    /// To trigger a SignCertificate with typeOfCertificate: ChargingStationCertificate.
    SignChargingStationCertificate,
    /// To trigger a SignCertificate with typeOfCertificate: V2GCertificate
    SignV2GCertificate,
    /// Same as SignV2Gcertificate, but this triggers Charging Station explicitly to only sign V2G certificate for ISO 15118-20.
    SignV2G20Certificate,
    /// To trigger StatusNotification.
    StatusNotification,
    /// To trigger TransactionEvent.
    TransactionEvent,
    /// To trigger a SignCertificate with typeOfCertificate: ChargingStationCertificate AND V2GCertificate
    SignCombinedCertificate,
    /// To trigger PublishFirmwareStatusNotification.
    PublishFirmwareStatusNotification,
    /// (2.1) To trigger the message referred to in customTrigger field.
    CustomTrigger,
}

impl fmt::Display for MessageTriggerEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BootNotification => write!(f, "BootNotification"),
            Self::LogStatusNotification => write!(f, "LogStatusNotification"),
            Self::FirmwareStatusNotification => write!(f, "FirmwareStatusNotification"),
            Self::Heartbeat => write!(f, "Heartbeat"),
            Self::MeterValues => write!(f, "MeterValues"),
            Self::SignChargingStationCertificate => write!(f, "SignChargingStationCertificate"),
            Self::SignV2GCertificate => write!(f, "SignV2GCertificate"),
            Self::SignV2G20Certificate => write!(f, "SignV2G20Certificate"),
            Self::StatusNotification => write!(f, "StatusNotification"),
            Self::TransactionEvent => write!(f, "TransactionEvent"),
            Self::SignCombinedCertificate => write!(f, "SignCombinedCertificate"),
            Self::PublishFirmwareStatusNotification => {
                write!(f, "PublishFirmwareStatusNotification")
            }
            Self::CustomTrigger => write!(f, "CustomTrigger"),
        }
    }
}

impl Into<String> for MessageTriggerEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MessageTriggerEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BootNotification" => Ok(Self::BootNotification),
            "LogStatusNotification" => Ok(Self::LogStatusNotification),
            "FirmwareStatusNotification" => Ok(Self::FirmwareStatusNotification),
            "Heartbeat" => Ok(Self::Heartbeat),
            "MeterValues" => Ok(Self::MeterValues),
            "SignChargingStationCertificate" => Ok(Self::SignChargingStationCertificate),
            "SignV2GCertificate" => Ok(Self::SignV2GCertificate),
            "SignV2G20Certificate" => Ok(Self::SignV2G20Certificate),
            "StatusNotification" => Ok(Self::StatusNotification),
            "TransactionEvent" => Ok(Self::TransactionEvent),
            "SignCombinedCertificate" => Ok(Self::SignCombinedCertificate),
            "PublishFirmwareStatusNotification" => Ok(Self::PublishFirmwareStatusNotification),
            "CustomTrigger" => Ok(Self::CustomTrigger),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MessageTriggerEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
