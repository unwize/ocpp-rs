use miette::Diagnostic;
use thiserror::Error;

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
        cardinality: i32,
        lower: i32,
        upper: i32,
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