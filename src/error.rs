// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use std::fmt::{self, Display};
use thiserror::Error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
/// Error type
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

/// Possible errors encountered while hashing/encoding an EIP-712 compliant data structure
#[derive(Clone, Error, Debug, PartialEq)]
pub enum ErrorKind {
    /// if we fail to deserialize from a serde::Value as a type specified in message types
    /// fail with this error.
    #[error("Expected type '{0}' for field '{1}'")]
    UnexpectedType(String, String),
    /// the primary type supplied doesn't exist in the MessageTypes
    #[error("The given primaryType wasn't found in the types field")]
    NonExistentType,
    /// an invalid address was encountered during encoding
    #[error("Address string should be a 0x-prefixed 40 character string, got '{0}'")]
    InvalidAddressLength(usize),
    /// a hex parse error occured
    #[error("Failed to parse hex '{0}'")]
    HexParseError(String),
    /// the field was declared with a unknown type
    #[error("The field '{0}' has an unknown type '{1}'")]
    UnknownType(String, String),
    /// Unexpected token
    #[error("Unexpected token '{0}' while parsing typename '{1}'")]
    UnexpectedToken(String, String),
    /// the user has attempted to define a typed array with a depth > 10
    #[error("Maximum depth for nested arrays is 10")]
    UnsupportedArrayDepth,
    /// FieldType validation error
    #[error("{0}")]
    ValidationError(String),
    /// the typed array defined in message types was declared with a fixed length
    /// that is of unequal length with the items to be encoded
    #[error("Expected {0} items for array type {1}, got {2} items")]
    UnequalArrayItems(u64, String, u64),
    /// Typed array length doesn't fit into a u64
    #[error("Attempted to declare fixed size with length {0}")]
    InvalidArraySize(String),
    /// Error occurred while parsing the type
    #[error("Parse type faild {0}")]
    LexerError(String),
}

pub(crate) fn serde_error(expected: &str, field: Option<&str>) -> ErrorKind {
    ErrorKind::UnexpectedType(expected.to_owned(), field.unwrap_or("").to_owned())
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    /// extract the error kind
    pub fn kind(&self) -> ErrorKind {
        self.inner.clone()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: kind }
    }
}

impl From<ValidationErrors> for Error {
    fn from(error: ValidationErrors) -> Self {
        let mut string: String = "".into();
        for (field_name, error_kind) in error.errors() {
            match error_kind {
                ValidationErrorsKind::Field(validation_errors) => {
                    for error in validation_errors {
                        let str_error = format!(
                            "the field '{}', has an invalid value {}",
                            field_name, error.params["value"]
                        );
                        string.push_str(&str_error);
                    }
                }
                _ => unreachable!(
                    "#[validate] is only used on fields for regex;\
				its impossible to get any other	ErrorKind; qed"
                ),
            }
        }
        ErrorKind::ValidationError(string).into()
    }
}
