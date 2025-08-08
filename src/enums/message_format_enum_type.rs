use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use crate::errors::OcppError;

/// Format of a message to be displayed on the display of the Charging Station.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MessageFormatEnumType {
    /// Message content is ASCII formatted, only 7-bit printable ASCII allowed.
    ASCII,
    /// Message content is HTML formatted.
    HTML,
    /// Message content is URI that Charging Station should download and use to display; for example a HTML page to be shown in a web-browser.
    URI,
    /// Message content is UTF-8 formatted.
    UTF8,
    /// Message content is a text (usually a URL) that Charging Station will display as a QR code on the display. Note: this is not a dynamic QR code and should not be used for payments.
    QRCODE,
}

impl fmt::Display for MessageFormatEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ASCII => write!(f, "ASCII"),
            Self::HTML => write!(f, "HTML"),
            Self::URI => write!(f, "URI"),
            Self::UTF8 => write!(f, "UTF8"),
            Self::QRCODE => write!(f, "QRCODE"),
        }
    }
}

impl Into<String> for MessageFormatEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MessageFormatEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ASCII" => Ok(Self::ASCII),
            "HTML" => Ok(Self::HTML),
            "URI" => Ok(Self::URI),
            "UTF8" => Ok(Self::UTF8),
            "QRCODE" => Ok(Self::QRCODE),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MessageFormatEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}