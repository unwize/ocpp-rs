use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum HashAlgorithmEnumType {
    /// SHA-256 hash algorithm.
    SHA256,
    /// SHA-384 hash algorithm.
    SHA384,
    /// SHA-512 hash algorithm.
    SHA512,
}

impl TryFrom<String> for HashAlgorithmEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "SHA256" => Ok(HashAlgorithmEnumType::SHA256),
            "SHA384" => Ok(HashAlgorithmEnumType::SHA384),
            "SHA512" => Ok(HashAlgorithmEnumType::SHA512),
            _ => Err(format!("'{}' is not a valid HashAlgorithmEnumType", s)),
        }
    }
}

impl From<HashAlgorithmEnumType> for String {
    fn from(val: HashAlgorithmEnumType) -> Self {
        match val {
            HashAlgorithmEnumType::SHA256 => "SHA256".to_string(),
            HashAlgorithmEnumType::SHA384 => "SHA384".to_string(),
            HashAlgorithmEnumType::SHA512 => "SHA512".to_string(),
        }
    }
}
