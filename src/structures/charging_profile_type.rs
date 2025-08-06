use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::enums::charging_profile_kind_enum_type::ChargingProfileKindEnumType;
use crate::enums::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_schedule_type::ChargingScheduleType;
use crate::traits::OcppEntity;

/// Represents a charging profile.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingProfileType {
    /// Required. Id of ChargingProfile. Unique within Charging Station.
    /// Id can have a negative value. This is used to reference charging profiles from an external actor
    /// (external constraints) from charging profiles received from CSMS.
    pub id: i32,

    /// Required. Value determining level in hierarchy stack of profiles.
    /// Higher values have precedence over lower values. Lowest level is 0.
    /// Constraints: 0 <= val
    pub stack_level: i32,

    /// Required. Defines the purpose of the schedule transferred by this profile.
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    /// Required. Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKindEnumType,
    /// Optional. Indicates start point of a recurrence.
    pub recurrence_kind: Option<RecurrenceKindEnumType>, // TODO: Implement RecurrenceKindEnumType
    /// Optional. Point in time at which the profile starts to be valid.
    /// If absent, the profile is valid as soon as it is received by the Charging Station.
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. Point in time at which the profile stops to be valid.
    /// If absent, the profile is valid until it is replaced by another profile.
    pub valid_to: Option<DateTime<Utc>>,

    /// Optional. SHALL only be included if ChargingProfilePurpose is set to TxProfile in a SetChargingProfileRequest.
    /// The IdTokenId is used to match the profile to a specific transaction.
    /// String length: 0..36
    pub transaction_id: Option<String>,

    /// Optional. Period in seconds that this charging profile remains valid after the Charging Station has gone offline.
    /// After this period, the charging profile is invalid for as long as it is offline and the Charging Station
    /// reverts back to a valid profile with a lower stack level. If the Charging Station is online again,
    /// the charging profile will become permanently invalid. A value of 0 means that the charging profile
    /// remains valid while offline. When the profile is absent, then no timeout applies and the charging profile remains valid when offline.
    pub max_offline_duration: Option<i32>,

    /// Optional. When set to true this charging profile will not be valid anymore after being offline for more than maxOfflineDuration.
    /// When absent defaults to false.
    pub invalid_after_offline_duration: Option<bool>,

    /// Optional. Interval in seconds after receipt of last update, when to request a profile update by sending a PullDynamicScheduleUpdateRequest message.
    /// A value of 0 or less means that no update interval applies. Only relevant in a dynamic charging profile.
    pub dyn_update_interval: Option<i32>,

    /// Optional. Time at which limits or setpoints in this charging profile are to be updated by a PullDynamicScheduleUpdateRequest or
    /// UpdateDynamicScheduleRequest by an external actor. Only relevant in a dynamic charging profile.
    pub dyn_update_time: Option<DateTime<Utc>>,

    /// Optional. ISO 15118-20 signature for all price schedules or ChargingSchedules.
    /// The value of the signature (like secp256k1) the ECDSA e.g. signature is 612 bits (64 bytes) and for 521 is 132 bytes.
    /// The value of the signature (like secp256k1) the ECDSA e.g. signature is 612 bits (64 bytes) and for 521 is 132 bytes.
    /// This equals 131 bytes, which can be encoded as base64 in 176 bytes.
    /// String length: 0..256
    pub price_schedule_signature: Option<String>,

    /// Required. Schedule that contains limits for the available power or current over time.
    /// In order to support ISO 15118 schedules, the Charging Station SHALL support these schedules with associated tariff to choose from.
    /// Having multiple ChargingSchedules is only allowed for ChargingProfiles of type TxProfile, i.e. in the context of an ISO 15118 charging session.
    /// For ISO 15118 Dynamic Control Mode (AC_EVSECC), only one ChargingSchedule is allowed.
    pub charging_schedule: ChargingScheduleType,
}

impl OcppEntity for ChargingProfileType {
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_bounds("stack_level", 0, i32::MAX, self.stack_level);

        if let Some(transaction_id) = &self.transaction_id {
            e.check_cardinality("transaction_id", 0, 36, &transaction_id.chars());

            if self.charging_profile_purpose != ChargingProfilePurposeEnumType::TxProfile  {
                e.push_relation_error(
                    "transaction_id",
                    "charging_profile_purpose",
                    "transaction_id SHALL only be included if ChargingProfilePurpose is set to TxProfile in a SetChargingProfileRequest."
                );
            }
        }

        if let Some(max_offline_duration) = &self.max_offline_duration {
            e.check_bounds("max_offline_duration", 0, i32::MAX, *max_offline_duration);
        }

        if let Some(price_schedule_signature) = &self.price_schedule_signature {
            e.check_cardinality("price_schedule_signature", 0, 256, &price_schedule_signature.chars());
        }

        e.check_member("charging_schedule", &self.charging_schedule);

        e.build("ChargingProfileType")
    }
}