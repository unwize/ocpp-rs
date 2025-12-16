use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DayOfWeekEnumType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl TryFrom<String> for DayOfWeekEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Monday" => Ok(DayOfWeekEnumType::Monday),
            "Tuesday" => Ok(DayOfWeekEnumType::Tuesday),
            "Wednesday" => Ok(DayOfWeekEnumType::Wednesday),
            "Thursday" => Ok(DayOfWeekEnumType::Thursday),
            "Friday" => Ok(DayOfWeekEnumType::Friday),
            "Saturday" => Ok(DayOfWeekEnumType::Saturday),
            "Sunday" => Ok(DayOfWeekEnumType::Sunday),
            _ => Err(format!("'{}' is not a valid DayOfWeekEnumType", s)),
        }
    }
}

impl From<DayOfWeekEnumType> for String {
    fn from(val: DayOfWeekEnumType) -> Self {
        match val {
            DayOfWeekEnumType::Monday => "Monday".to_string(),
            DayOfWeekEnumType::Tuesday => "Tuesday".to_string(),
            DayOfWeekEnumType::Wednesday => "Wednesday".to_string(),
            DayOfWeekEnumType::Thursday => "Thursday".to_string(),
            DayOfWeekEnumType::Friday => "Friday".to_string(),
            DayOfWeekEnumType::Saturday => "Saturday".to_string(),
            DayOfWeekEnumType::Sunday => "Sunday".to_string(),
        }
    }
}
