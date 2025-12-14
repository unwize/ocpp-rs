use std::fmt::Debug;
use dyn_clone::DynClone;
use crate::errors::OcppError;

#[typetag::serde(tag = "type")]  // Permit serde-ing of `dyn OcppEntity`
pub trait OcppEntity: Debug + DynClone {
    fn validate(self: &Self) -> Result<(), OcppError>;
}

dyn_clone::clone_trait_object!(OcppEntity); // Permit impl Clone of `dyn OcppEntity`


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
