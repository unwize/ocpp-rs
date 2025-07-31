use serde::{Deserialize, Serialize};

/// A generic address format.
/// Used by: NotifySettlementRequest, VatNumberValidationResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressType {
    /// Required. Name of person/company.
    /// String length: 0..50
    pub name: String,
    /// Required. Address line 1.
    /// String length: 0..100
    pub address1: String,
    /// Optional. Address line 2.
    /// String length: 0..100
    pub address2: Option<String>,
    /// Required. City.
    /// String length: 0..100
    pub city: String,
    /// Optional. Postal code.
    /// String length: 0..20
    pub postal_code: Option<String>,
    /// Required. Country name.
    /// String length: 0..50
    pub country: String,
}

impl AddressType {
    /// Validates the fields of AddressType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Validate required fields
        if self.name.len() > 50 {
            // println!("Validation failed: name length exceeds 50.");
            return false;
        }
        if self.address1.len() > 100 {
            // println!("Validation failed: address1 length exceeds 100.");
            return false;
        }
        if self.city.len() > 100 {
            // println!("Validation failed: city length exceeds 100.");
            return false;
        }
        if self.country.len() > 50 {
            // println!("Validation failed: country length exceeds 50.");
            return false;
        }

        // Validate optional fields if they exist
        if let Some(addr2) = &self.address2 {
            if addr2.len() > 100 {
                // println!("Validation failed: address2 length exceeds 100.");
                return false;
            }
        }
        if let Some(postal_code) = &self.postal_code {
            if postal_code.len() > 20 {
                // println!("Validation failed: postal_code length exceeds 20.");
                return false;
            }
        }

        true
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "123 Main St".to_string(),
            address2: Some("Apt 4B".to_string()),
            city: "Anytown".to_string(),
            postal_code: Some("12345".to_string()),
            country: "USA".to_string(),
        };

        let serialized = serde_json::to_string(&address).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: AddressType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(address, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let address = AddressType {
            name: "Jane Smith".to_string(),
            address1: "456 Oak Ave".to_string(),
            address2: None,
            city: "Someville".to_string(),
            postal_code: None,
            country: "Canada".to_string(),
        };
        assert!(address.validate());

        let address_with_all_fields = AddressType {
            name: "Max Name".repeat(5), // 50 chars
            address1: "Max Address Line 1".repeat(5), // 100 chars
            address2: Some("Max Address Line 2".repeat(5)), // 100 chars
            city: "Max City".repeat(10), // 100 chars
            postal_code: Some("12345678901234567890".to_string()), // 20 chars
            country: "Max Country".repeat(4), // 44 chars
        };
        assert!(address_with_all_fields.validate());
    }

    #[test]
    fn test_validation_name_too_long() {
        let address = AddressType {
            name: "a".repeat(51), // Too long
            address1: "123 Main St".to_string(),
            address2: None,
            city: "Anytown".to_string(),
            postal_code: None,
            country: "USA".to_string(),
        };
        assert!(!address.validate());
    }

    #[test]
    fn test_validation_address1_too_long() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "a".repeat(101), // Too long
            address2: None,
            city: "Anytown".to_string(),
            postal_code: None,
            country: "USA".to_string(),
        };
        assert!(!address.validate());
    }

    #[test]
    fn test_validation_address2_too_long() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "123 Main St".to_string(),
            address2: Some("a".repeat(101)), // Too long
            city: "Anytown".to_string(),
            postal_code: None,
            country: "USA".to_string(),
        };
        assert!(!address.validate());
    }

    #[test]
    fn test_validation_city_too_long() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "123 Main St".to_string(),
            address2: None,
            city: "a".repeat(101), // Too long
            postal_code: None,
            country: "USA".to_string(),
        };
        assert!(!address.validate());
    }

    #[test]
    fn test_validation_postal_code_too_long() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "123 Main St".to_string(),
            address2: None,
            city: "Anytown".to_string(),
            postal_code: Some("a".repeat(21)), // Too long
            country: "USA".to_string(),
        };
        assert!(!address.validate());
    }

    #[test]
    fn test_validation_country_too_long() {
        let address = AddressType {
            name: "John Doe".to_string(),
            address1: "123 Main St".to_string(),
            address2: None,
            city: "Anytown".to_string(),
            postal_code: None,
            country: "a".repeat(51), // Too long
        };
        assert!(!address.validate());
    }
}
