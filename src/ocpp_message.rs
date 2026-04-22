use crate::messages::{
    adjust_periodic_event_stream::{
        AdjustPeriodicEventStreamRequest, AdjustPeriodicEventStreamResponse,
    },
    afr_signal::{AFRRSignalRequest, AFRRSignalResponse},
    authorize::{AuthorizeRequest, AuthorizeResponse},
    battery_swap::{BatterySwapRequest, BatterySwapResponse},
    boot_notification::{BootNotificationRequest, BootNotificationResponse},
    cancel_reservation::{CancelReservationRequest, CancelReservationResponse},
    certificate_signed::{CertificateSignedRequest, CertificateSignedResponse},
    change_availability::{ChangeAvailabilityRequest, ChangeAvailabilityResponse},
    change_transaction_tariff::{ChangeTransactionTariffRequest, ChangeTransactionTariffResponse},
    clear_cache::{ClearCacheRequest, ClearCacheResponse},
    clear_charging_profile::{ClearChargingProfileRequest, ClearChargingProfileResponse},
    clear_der_control::{ClearDERControlRequest, ClearDERControlResponse},
    clear_display_message::{ClearDisplayMessageRequest, ClearDisplayMessageResponse},
    clear_tariffs::{ClearTariffsRequest, ClearTariffsResponse},
    clear_variable_monitoring::{ClearVariableMonitoringRequest, ClearVariableMonitoringResponse},
    cleared_charging_limit::{ClearedChargingLimitRequest, ClearedChargingLimitResponse},
    close_periodic_event_stream::{
        ClosePeriodicEventStreamRequest, ClosePeriodicEventStreamResponse,
    },
    cost_updated::{CostUpdatedRequest, CostUpdatedResponse},
    customer_information::{CustomerInformationRequest, CustomerInformationResponse},
    data_transfer::{DataTransferRequest, DataTransferResponse},
    delete_certificate::{DeleteCertificateRequest, DeleteCertificateResponse},
    firmware_status_notification::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    },
    get_15118_ev_certificate::{Get15118EVCertificateRequest, Get15118EVCertificateResponse},
    get_base_report::{GetBaseReportRequest, GetBaseReportResponse},
    get_certificate_chain_status::{
        GetCertificateChainStatusRequest, GetCertificateChainStatusResponse,
    },
    get_certificate_status::{GetCertificateStatusRequest, GetCertificateStatusResponse},
    get_charging_profiles::{GetChargingProfilesRequest, GetChargingProfilesResponse},
    get_composite_schedule::{GetCompositeScheduleRequest, GetCompositeScheduleResponse},
    get_der_control::{GetDERControlRequest, GetDERControlResponse},
    get_display_messages::{GetDisplayMessagesRequest, GetDisplayMessagesResponse},
    get_installed_certificate_ids::{
        GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    },
    get_local_list_version::{GetLocalListVersionRequest, GetLocalListVersionResponse},
    get_log::{GetLogRequest, GetLogResponse},
    get_monitoring_report::{GetMonitoringReportRequest, GetMonitoringReportResponse},
    get_periodic_event_stream::{GetPeriodicEventStreamRequest, GetPeriodicEventStreamResponse},
    get_report::{GetReportRequest, GetReportResponse},
    get_tariffs::{GetTariffsRequest, GetTariffsResponse},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppMessage {
    AdjustPeriodicEventStreamRequest(AdjustPeriodicEventStreamRequest),
    AdjustPeriodicEventStreamResponse(AdjustPeriodicEventStreamResponse),
    AFRRSignalRequest(AFRRSignalRequest),
    AFRRSignalResponse(AFRRSignalResponse),
    AuthorizeRequest(AuthorizeRequest),
    AuthorizeResponse(Box<AuthorizeResponse>),
    BatterySwapRequest(BatterySwapRequest),
    BatterySwapResponse(BatterySwapResponse),
    BootNotificationRequest(BootNotificationRequest),
    BootNotificationResponse(BootNotificationResponse),
    CancelReservationRequest(CancelReservationRequest),
    CancelReservationResponse(CancelReservationResponse),
    CertificateSignedRequest(CertificateSignedRequest),
    CertificateSignedResponse(CertificateSignedResponse),
    ChangeAvailabilityRequest(ChangeAvailabilityRequest),
    ChangeAvailabilityResponse(ChangeAvailabilityResponse),
    ChangeTransactionTariffRequest(Box<ChangeTransactionTariffRequest>),
    ChangeTransactionTariffResponse(ChangeTransactionTariffResponse),
    ClearCacheRequest(ClearCacheRequest),
    ClearCacheResponse(ClearCacheResponse),
    ClearChargingProfileRequest(ClearChargingProfileRequest),
    ClearChargingProfileResponse(ClearChargingProfileResponse),
    ClearDERControlRequest(ClearDERControlRequest),
    ClearDERControlResponse(ClearDERControlResponse),
    ClearDisplayMessageRequest(ClearDisplayMessageRequest),
    ClearDisplayMessageResponse(ClearDisplayMessageResponse),
    ClearTariffsRequest(ClearTariffsRequest),
    ClearTariffsResponse(ClearTariffsResponse),
    ClearVariableMonitoringRequest(ClearVariableMonitoringRequest),
    ClearVariableMonitoringResponse(ClearVariableMonitoringResponse),
    ClearedChargingLimitRequest(ClearedChargingLimitRequest),
    ClearedChargingLimitResponse(ClearedChargingLimitResponse),
    ClosePeriodicEventStreamRequest(ClosePeriodicEventStreamRequest),
    ClosePeriodicEventStreamResponse(ClosePeriodicEventStreamResponse),
    CostUpdatedRequest(CostUpdatedRequest),
    CostUpdatedResponse(CostUpdatedResponse),
    CustomerInformationRequest(CustomerInformationRequest),
    CustomerInformationResponse(CustomerInformationResponse),
    DataTransferRequest(DataTransferRequest),
    DataTransferResponse(DataTransferResponse),
    DeleteCertificateRequest(DeleteCertificateRequest),
    DeleteCertificateResponse(DeleteCertificateResponse),
    FirmwareStatusNotificationRequest(FirmwareStatusNotificationRequest),
    FirmwareStatusNotificationResponse(FirmwareStatusNotificationResponse),
    Get15118EVCertificateRequest(Get15118EVCertificateRequest),
    Get15118EVCertificateResponse(Get15118EVCertificateResponse),
    GetBaseReportRequest(GetBaseReportRequest),
    GetBaseReportResponse(GetBaseReportResponse),
    GetCertificateChainStatusRequest(GetCertificateChainStatusRequest),
    GetCertificateChainStatusResponse(GetCertificateChainStatusResponse),
    GetCertificateStatusRequest(GetCertificateStatusRequest),
    GetCertificateStatusResponse(GetCertificateStatusResponse),
    GetChargingProfilesRequest(GetChargingProfilesRequest),
    GetChargingProfilesResponse(GetChargingProfilesResponse),
    GetCompositeScheduleRequest(GetCompositeScheduleRequest),
    GetCompositeScheduleResponse(GetCompositeScheduleResponse),
    GetDERControlRequest(GetDERControlRequest),
    GetDERControlResponse(GetDERControlResponse),
    GetDisplayMessagesRequest(GetDisplayMessagesRequest),
    GetDisplayMessagesResponse(GetDisplayMessagesResponse),
    GetInstalledCertificateIdsRequest(GetInstalledCertificateIdsRequest),
    GetInstalledCertificateIdsResponse(GetInstalledCertificateIdsResponse),
    GetLocalListVersionRequest(GetLocalListVersionRequest),
    GetLocalListVersionResponse(GetLocalListVersionResponse),
    GetLogRequest(GetLogRequest),
    GetLogResponse(GetLogResponse),
    GetMonitoringReportRequest(GetMonitoringReportRequest),
    GetMonitoringReportResponse(GetMonitoringReportResponse),
    GetPeriodicEventStreamRequest(GetPeriodicEventStreamRequest),
    GetPeriodicEventStreamResponse(GetPeriodicEventStreamResponse),
    GetReportRequest(GetReportRequest),
    GetReportResponse(GetReportResponse),
    GetTariffsRequest(GetTariffsRequest),
    GetTariffsResponse(GetTariffsResponse),
}

impl OcppMessage {
    pub fn get_message_name(&self) -> &'static str {
        match self {
            OcppMessage::AdjustPeriodicEventStreamRequest(_)
            | OcppMessage::AdjustPeriodicEventStreamResponse(_) => "AdjustPeriodicEventStream",
            OcppMessage::AFRRSignalRequest(_) | OcppMessage::AFRRSignalResponse(_) => "AFRRSignal",
            OcppMessage::AuthorizeRequest(_) | OcppMessage::AuthorizeResponse(_) => "Authorize",
            OcppMessage::BatterySwapRequest(_) | OcppMessage::BatterySwapResponse(_) => {
                "BatterySwap"
            }
            OcppMessage::BootNotificationRequest(_) | OcppMessage::BootNotificationResponse(_) => {
                "BootNotification"
            }
            OcppMessage::CancelReservationRequest(_)
            | OcppMessage::CancelReservationResponse(_) => "CancelReservation",
            OcppMessage::CertificateSignedRequest(_)
            | OcppMessage::CertificateSignedResponse(_) => "CertificateSigned",
            OcppMessage::ChangeAvailabilityRequest(_)
            | OcppMessage::ChangeAvailabilityResponse(_) => "ChangeAvailability",
            OcppMessage::ChangeTransactionTariffRequest(_)
            | OcppMessage::ChangeTransactionTariffResponse(_) => "ChangeTransactionTariff",
            OcppMessage::ClearCacheRequest(_) | OcppMessage::ClearCacheResponse(_) => "ClearCache",
            OcppMessage::ClearChargingProfileRequest(_)
            | OcppMessage::ClearChargingProfileResponse(_) => "ClearChargingProfile",
            OcppMessage::ClearDERControlRequest(_) | OcppMessage::ClearDERControlResponse(_) => {
                "ClearDERControl"
            }
            OcppMessage::ClearDisplayMessageRequest(_)
            | OcppMessage::ClearDisplayMessageResponse(_) => "ClearDisplayMessage",
            OcppMessage::ClearTariffsRequest(_) | OcppMessage::ClearTariffsResponse(_) => {
                "ClearTariffs"
            }
            OcppMessage::ClearVariableMonitoringRequest(_)
            | OcppMessage::ClearVariableMonitoringResponse(_) => "ClearVariableMonitoring",
            OcppMessage::ClearedChargingLimitRequest(_)
            | OcppMessage::ClearedChargingLimitResponse(_) => "ClearedChargingLimit",
            OcppMessage::ClosePeriodicEventStreamRequest(_)
            | OcppMessage::ClosePeriodicEventStreamResponse(_) => "ClosePeriodicEventStream",
            OcppMessage::CostUpdatedRequest(_) | OcppMessage::CostUpdatedResponse(_) => {
                "CostUpdated"
            }
            OcppMessage::CustomerInformationRequest(_)
            | OcppMessage::CustomerInformationResponse(_) => "CustomerInformation",
            OcppMessage::DataTransferRequest(_) | OcppMessage::DataTransferResponse(_) => {
                "DataTransfer"
            }
            OcppMessage::DeleteCertificateRequest(_)
            | OcppMessage::DeleteCertificateResponse(_) => "DeleteCertificate",
            OcppMessage::FirmwareStatusNotificationRequest(_)
            | OcppMessage::FirmwareStatusNotificationResponse(_) => "FirmwareStatusNotification",
            OcppMessage::Get15118EVCertificateRequest(_)
            | OcppMessage::Get15118EVCertificateResponse(_) => "Get15118EVCertificate",
            OcppMessage::GetBaseReportRequest(_) | OcppMessage::GetBaseReportResponse(_) => {
                "GetBaseReport"
            }
            OcppMessage::GetCertificateChainStatusRequest(_)
            | OcppMessage::GetCertificateChainStatusResponse(_) => "GetCertificateChainStatus",
            OcppMessage::GetCertificateStatusRequest(_)
            | OcppMessage::GetCertificateStatusResponse(_) => "GetCertificateStatus",
            OcppMessage::GetChargingProfilesRequest(_)
            | OcppMessage::GetChargingProfilesResponse(_) => "GetChargingProfiles",
            OcppMessage::GetCompositeScheduleRequest(_)
            | OcppMessage::GetCompositeScheduleResponse(_) => "GetCompositeSchedule",
            OcppMessage::GetDERControlRequest(_) | OcppMessage::GetDERControlResponse(_) => {
                "GetDERControl"
            }
            OcppMessage::GetDisplayMessagesRequest(_)
            | OcppMessage::GetDisplayMessagesResponse(_) => "GetDisplayMessages",
            OcppMessage::GetInstalledCertificateIdsRequest(_)
            | OcppMessage::GetInstalledCertificateIdsResponse(_) => "GetInstalledCertificateIds",
            OcppMessage::GetLocalListVersionRequest(_)
            | OcppMessage::GetLocalListVersionResponse(_) => "GetLocalListVersion",
            OcppMessage::GetLogRequest(_) | OcppMessage::GetLogResponse(_) => "GetLog",
            OcppMessage::GetMonitoringReportRequest(_)
            | OcppMessage::GetMonitoringReportResponse(_) => "GetMonitoringReport",
            OcppMessage::GetPeriodicEventStreamRequest(_)
            | OcppMessage::GetPeriodicEventStreamResponse(_) => "GetPeriodicEventStream",
            OcppMessage::GetReportRequest(_) | OcppMessage::GetReportResponse(_) => "GetReport",
            OcppMessage::GetTariffsRequest(_) | OcppMessage::GetTariffsResponse(_) => "GetTariffs",
        }
    }
}
