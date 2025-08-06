use crate::errors::OcppError;

pub trait OcppEntity {
    fn validate(self: &Self) -> Result<(), OcppError>;
}