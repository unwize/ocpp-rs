use crate::errors::OcppError;
use std::fmt::Debug;

pub trait OcppEntity: Debug {
    fn validate(&self) -> Result<(), OcppError>;
}

pub trait OcppRequest: Debug {
    fn get_message_type(&self) -> String;
}

pub trait OcppMessage {
    type Request: Default;
    type Response: Default;

    fn request() -> Self::Request {
        Self::Request::default()
    }
    fn response() -> Self::Response {
        Self::Response::default()
    }
}
