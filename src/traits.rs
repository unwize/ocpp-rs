use crate::errors::OcppError;

pub trait OcppEntity {
    fn validate(self: &Self) -> Result<(), OcppError>;
}

pub trait OcppMessage {
    type Request: Default;
    type Response: Default;

    fn request(&self) -> &Self::Request {
        Self::Request::default()
    }
    fn response(&self) -> &Self::Response {
        Self::Response::default()
    }
}