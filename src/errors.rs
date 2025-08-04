use std::slice::Iter;
use miette::Diagnostic;
use thiserror::Error;
use crate::errors::OcppError::{FieldBoundsError, FieldCardinalityError, FieldValidationError, StructureValidationError};
use crate::traits::OcppEntity;

#[derive(Error, Diagnostic, Debug)]
pub enum OcppError {

    #[error("Invalid Enum Value: {value} not in {enum_name}")]
    #[diagnostic()]
    InvalidEnumValueError {
        enum_name: String,
        value: String,
    },

    #[error("OCPP Structure Validation Error: {structure")]
    #[diagnostic()]
    StructureValidationError {
        structure: String,

        #[related]
        source: Vec<OcppError>
    },

    #[error("OCPP Field Validation Error: {field}")]
    #[diagnostic()]
    FieldValidationError  {
        field: String,

        #[related]
        source: Vec<OcppError>,
    },

    #[error("Field Cardinality Error: {cardinality} not in range {lower}..{upper}")]
    #[diagnostic()]
    FieldCardinalityError {
        cardinality: usize,
        lower: usize,
        upper: usize,
    },

    #[error("Field Bound Error: {value} not in range {lower}..{upper}")]
    #[diagnostic()]
    FieldBoundsError {
        value: String,
        lower: String,
        upper: String,
    },

    #[error("Field Value Error: {value} is not a valid value")]
    #[diagnostic()]
    FieldValueError {
        value: String,
    },

    #[error("Field ISO Error: {field} does not comply with ISO {iso}")]
    #[diagnostic()]
    FieldISOError {
        value: String,
        iso: String,
    }
}

impl OcppError {

    /// A convenience function that consumes any OcppError and returns it wrapped in a FieldValidationError
    pub fn to_field_validation_error(self, field: &str) -> OcppError {
        FieldValidationError {
            field: field.to_string(),
            source: vec![self],
        }
    }
}

/// Convenience function to read a StructureValidationError, parse its sources, and verify that the
/// provided vec of field names appear in the vec of sources. Each field is asserted to appear.
pub fn assert_invalid_fields(e: OcppError, fields: Vec<String>) {
    if let OcppError::StructureValidationError { source, .. } = e {
        let field_names: Vec<String> = source.iter().map(|e| {
            if let OcppError::FieldValidationError { field, .. } = e {
                field.clone()
            } else {
                "".to_string()
            }
        }).collect();

        for field in &fields {
            assert!(field_names.contains(field))
        }
    } else {
        panic!("Expected StructureValidationError");
    }
}

/// Convenience function to check the length of a string and throw an error if it is out of range.
pub fn validate_string_length(s: &str, min_len: usize, max_len: usize) -> Result<(), OcppError> {
    if s.len() < min_len || s.len() > max_len {
        return Err(OcppError::FieldBoundsError {
            value: s.to_string(),
            lower: min_len.to_string(),
            upper: max_len.to_string(),
        })
    }

    Ok(())
}

/// A builder that performs validation and accumulation of fields in an arbitrary order
pub struct StructureValidationBuilder {
    errors: Vec<OcppError>,
}

impl StructureValidationBuilder {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    /// Directly push an error to the list.
    pub fn push(mut self, error: OcppError) -> Self {
        self.errors.push(error);
        self
    }

    /// For a given OcppEntity, call its validate function and add any errors from it to the list.
    pub fn push_member(mut self, field: &str, member: &dyn OcppEntity) -> Self {
        if let Err(e) = member.validate() {
            self.errors.push(e.to_field_validation_error(field));
        }

        self
    }

    /// For a given field, check if its value is within the given bounds. If it is not, add a
    /// `OcppError::FieldBoundError` to the list.
    pub fn check_bounds<T: Ord>(mut self, field: &str, min: T, max: T, value: T) -> Self {
        if value < min || value > max {
            self.errors.push(FieldBoundsError {
                value,
                lower: min,
                upper: max,
            }.to_field_validation_error(field));
        }

        self
    }

    /// For a given field and its value, check if its length is within the given range. If it is
    /// not, add a `OcppError::FieldCardinalityError` to the list.
    pub fn check_cardinality<T: ExactSizeIterator>(mut self, field: &str, lower: usize, upper: usize, o: &T,) -> Self {
        if o.len() < lower || o.len() > upper {
            self.errors.push(FieldCardinalityError {
                cardinality: o.len(),
                lower,
                upper,
            }.to_field_validation_error(field));
        }

        self
    }
}