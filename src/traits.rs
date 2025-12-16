use crate::errors::OcppError;
use dyn_clone::DynClone;
use std::fmt::Debug;

#[typetag::serde(tag = "type")] // Permit serde-ing of `dyn OcppEntity`
pub trait OcppEntity: Debug + DynClone {
    fn validate(&self) -> Result<(), OcppError>;
}

dyn_clone::clone_trait_object!(OcppEntity); // Permit impl Clone of `dyn OcppEntity`

#[typetag::serde(tag = "type")] // Permit serde-ing of `dyn OcppEntity`

pub trait OcppRequest: Debug + DynClone {
    fn get_message_type(&self) -> String;
}

dyn_clone::clone_trait_object!(OcppRequest); // Permit impl Clone of `dyn OcppEntity`

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
