use crate::errors::OcppError;

#[typetag::serde(tag = "type")]
pub trait OcppEntity {
    fn validate(self: &Self) -> Result<(), OcppError>;
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
