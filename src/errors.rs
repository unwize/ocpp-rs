use crate::errors::OcppError::{
    FieldBoundsError, FieldCardinalityError, FieldRelationshipError, FieldValidationError,
    StructureValidationError,
};
use crate::traits::OcppEntity;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum OcppError {
    #[error("Invalid Enum Value: {value} not in {enum_name}")]
    #[diagnostic()]
    InvalidEnumValueError { enum_name: String, value: String },

    #[error("OCPP Structure Validation Error: {structure}")]
    #[diagnostic()]
    StructureValidationError {
        structure: String,

        #[related]
        related: Vec<OcppError>,
    },

    #[error("OCPP Field Validation Error: {field}")]
    #[diagnostic()]
    FieldValidationError {
        field: String,

        #[related]
        related: Vec<OcppError>,
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
    FieldValueError { value: String },

    #[error("Field ISO Error: {value} does not comply with ISO {iso}")]
    #[diagnostic()]
    FieldISOError { value: String, iso: String },

    #[error("Field Relationship Error: {this} is invalid in relation to {other}")]
    #[diagnostic(help("{help}"))]
    FieldRelationshipError {
        this: String,
        other: String,
        help: String,
    },

    #[error("Builder Error: {builder_type} encountered an error while building!")]
    #[diagnostic(help("{help}"))]
    BuilderError { builder_type: String, help: String },
}

impl OcppError {
    /// A convenience function that consumes any OcppError and returns it wrapped in a FieldValidationError
    pub fn to_field_validation_error(self, field: &str) -> OcppError {
        FieldValidationError {
            field: field.to_string(),
            related: vec![self],
        }
    }
}

/// Convenience function to read a StructureValidationError, parse its sources, and verify that the
/// provided vec of field names appear in the vec of sources. Each field is asserted to appear.
pub fn assert_invalid_fields(e: &OcppError, fields: &[&str]) {
    match e {
        StructureValidationError { related, .. } => {
            let related_fields: Vec<String> = related
                .iter()
                .map(|e| match e {
                    FieldValidationError { field, .. } => field.clone(),
                    _ => "".to_string(),
                })
                .collect();
            for field in fields {
                assert!(
                    related_fields.contains(&field.to_string()),
                    "Expected field {} to throw an error!",
                    field
                );
            }
        }

        _ => {
            panic!("Expected a StructureValidationError. Got {e} instead.")
        }
    }
}

pub fn assert_num_field_errors(e: &OcppError, count: usize) {
    match e {
        StructureValidationError { related, .. } => {
            assert_eq!(related.len(), count)
        }

        _ => {
            panic!("Expected a StructureValidationError. Got {e} instead.")
        }
    }
}

/// Convenience function to check the length of a string and throw an error if it is out of range.
pub fn validate_string_length(s: &str, min_len: usize, max_len: usize) -> Result<(), OcppError> {
    if s.len() < min_len || s.len() > max_len {
        return Err(FieldBoundsError {
            value: s.to_string(),
            lower: min_len.to_string(),
            upper: max_len.to_string(),
        });
    }

    Ok(())
}

/// A builder that performs validation and accumulation of fields in an arbitrary order
pub struct StructureValidationBuilder {
    errors: Vec<OcppError>,
}

impl Default for StructureValidationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StructureValidationBuilder {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    /// Directly push an error to the list.
    pub fn push(&mut self, error: OcppError) -> &Self {
        self.errors.push(error);
        self
    }

    /// For a given OcppEntity, call its validate function and add any errors from it to the list.
    pub fn check_member(&mut self, field: &str, member: &dyn OcppEntity) -> &Self {
        if let Err(e) = member.validate() {
            self.errors.push(e.to_field_validation_error(field));
        }

        self
    }

    /// For a given Iterator of OcppEntity objects, call its yielded items' validate functions and add any errors from it to the list.
    pub fn check_iter_member<'a, T: OcppEntity + 'a>(
        &mut self,
        field: &str,
        iter: impl Iterator<Item = &'a T>,
    ) -> &Self {
        let mut count = 0;
        for e in iter {
            self.check_member(format!("{field}[{count}]").as_str(), e);
            count += 1;
        }
        self
    }

    /// For a given field, check if its value is within the given bounds. If it is not, add a
    /// `OcppError::FieldBoundError` to the list.
    pub fn check_bounds<T: PartialOrd + ToString>(
        &mut self,
        field: &str,
        min: T,
        max: T,
        value: T,
    ) -> &Self {
        if value < min || value > max {
            self.errors.push(
                FieldBoundsError {
                    value: value.to_string(),
                    lower: min.to_string(),
                    upper: max.to_string(),
                }
                .to_field_validation_error(field),
            );
        }

        self
    }

    /// For a given field and its value, check if its length is within the given range. If it is
    /// not, add a `OcppError::FieldCardinalityError` to the list.
    pub fn check_cardinality<T: Iterator>(
        &mut self,
        field: &str,
        lower: usize,
        upper: usize,
        o: &T,
    ) -> &Self {
        let len  = o.size_hint().1.expect("Something went wrong while checking an object's cardinality. `Iterator::size_hint` did not work as expected.");
        if len < lower || len > upper {
            self.errors.push(
                FieldCardinalityError {
                    cardinality: len,
                    lower,
                    upper,
                }
                .to_field_validation_error(field),
            );
        }

        self
    }

    pub fn push_relation_error(&mut self, this: &str, other: &str, help: &str) {
        let fre = FieldRelationshipError {
            this: this.to_string(),
            other: other.to_string(),
            help: help.to_string(),
        };
        self.errors
            .push(fre.clone().to_field_validation_error(this));
        self.errors.push(fre.to_field_validation_error(other));
    }

    pub fn build(&self, structure: &str) -> Result<(), OcppError> {
        if self.errors.is_empty() {
            return Ok(());
        }

        Err(StructureValidationError {
            structure: structure.to_string(),
            related: self.errors.clone(),
        })
    }
}
