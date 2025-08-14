use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalPriceType {
    /// Optional. Price/cost excluding tax. Can be absent if inclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl_tax: Option<f64>,
    /// Optional. Price/cost including tax. Can be absent if exclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_tax: Option<f64>,
}

impl Default for TotalPriceType {
    fn default() -> TotalPriceType {
        Self {
            excl_tax: Some(1.0),
            incl_tax: None,
        }
    }
}

impl OcppEntity for TotalPriceType {
    /// Validates the fields of TotalPriceType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(excl_tax) = self.excl_tax {
            e.check_bounds("excl_tax", 0.0, f64::MAX, excl_tax);
        }

        if let Some(incl_tax) = self.incl_tax {
            e.check_bounds("incl_tax", 0.0, f64::MAX, incl_tax);
        }

        // At least one of exclTax or inclTax must be present.
        if self.excl_tax.is_none() && self.incl_tax.is_none() {
            e.push_relation_error(
                "excl_tax",
                "incl_tax",
                "At least one of 'exclTax' or 'inclTax' must be present.",
            );
        }

        e.build("TotalPriceType")
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::*;
    use miette::Diagnostic;
    use serde_json;

    #[test]
    fn test_validate_failure_excl_tax_neg() {
        let data = TotalPriceType {
            excl_tax: Some(-1.0),
            incl_tax: None,
        };

        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_incl_tax_neg() {
        let data = TotalPriceType {
            excl_tax: None,
            incl_tax: Some(-1.0),
        };

        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_both_tax_neg() {
        let data = TotalPriceType {
            excl_tax: Some(-1.0),
            incl_tax: Some(-1.0),
        };
        assert!(data.validate().is_err());
        
        if let Err(e) = data.validate() {
            match e {
                OcppError::StructureValidationError {related, .. } => {
                    assert_eq!(related.len(), 2);
                }
                _ => assert!(false),
            }
        }
        
        
    }

    #[test]
    fn test_validate_success_excl_tax_present() {
        let data = TotalPriceType {
            excl_tax: Some(10.0),
            incl_tax: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_incl_tax_present() {
        let data = TotalPriceType {
            excl_tax: None,
            incl_tax: Some(12.0),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_both_present() {
        let data = TotalPriceType {
            excl_tax: Some(10.0),
            incl_tax: Some(12.0),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_neither_present() {
        let data = TotalPriceType {
            excl_tax: None,
            incl_tax: None,
        };
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TotalPriceType {
            excl_tax: Some(10.0),
            incl_tax: Some(12.0),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TotalPriceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
