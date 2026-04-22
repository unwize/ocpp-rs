use crate::errors::OcppError;
use std::fmt::Debug;

pub trait OcppEntity: Debug {
    fn validate(&self) -> Result<(), OcppError>;
}

pub trait OcppRequest: Debug {
    const NAME: &'static str;

    type ResponseType;

    fn get_message_type(&self) -> &'static str {
        Self::NAME
    }
}
