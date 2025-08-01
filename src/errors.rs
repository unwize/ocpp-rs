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

    #[error("Field Value Error: {value} not in range {lower}..{upper}")]
    #[diagnostic()]
    FieldValueError {
        value: String,
        lower: String,
        upper: String,
    }
}