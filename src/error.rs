/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! All the custom error types used in the crate.

use std::error::Error;
use std::fmt;
use std::fmt::Display;

//{ `field` module

/// Error type used in `Field::from_str`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseFieldError(pub(crate) String);
impl ParseFieldError {
    pub fn new(failed_title: &str) -> Self {
        Self(String::from(failed_title))
    }
}
impl Error for ParseFieldError {}
impl Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid field title '{}'", self.0)
    }
}

pub struct ParseFieldFilterError;
//}

//{ `format` module

/// Error type used in `Delimiter::new`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDelimiterError(pub(crate) char);
impl InvalidDelimiterError {
    pub fn new(failed_char: char) -> Self {
        Self(failed_char)
    }
}
impl Error for InvalidDelimiterError {}
impl Display for InvalidDelimiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid delimiter character '{}'", self.0.escape_debug())
    }
}

/// Error type used in `MissingValue::from_str`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseMissingError(pub(crate) String);
impl ParseMissingError {
    pub fn new(failed_value: &str) -> Self {
        Self(String::from(failed_value))
    }
}
impl Error for ParseMissingError {}
impl Display for ParseMissingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown missing value pattern \"{}\"", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvalidDelimiterSource {
    Error(InvalidDelimiterError),
    String(String),
}

/// Error type used in `FieldConfig::from_header`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseHeaderError {
    FoundNoDelimiters,
    FoundDuplicates(String),
    UnknownField(ParseFieldError),
    MissingTime,
    MissingStationNumber,
    InvalidEorPosition,
    InvalidTimeAlignment,
    InvalidDelimiter(InvalidDelimiterSource),
}
impl Error for ParseHeaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownField(e) => Some(e),
            Self::InvalidDelimiter(InvalidDelimiterSource::Error(e)) => Some(e),
            _ => None,
        }
    }
}
impl Display for ParseHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            Self::FoundNoDelimiters => "no delimiters found".to_string(),
            Self::FoundDuplicates(s) => format!("duplicate field found \"{}\"", s),
            Self::UnknownField(e) => e.to_string(),
            Self::MissingTime => "missing `Time` at position #1".to_string(),
            Self::MissingStationNumber => "missing `StationNumber` at position #2".to_string(),
            Self::InvalidEorPosition => "`EOR` must be last if present".to_string(),
            Self::InvalidTimeAlignment => "invalid characters detected before `Time`".to_string(),
            Self::InvalidDelimiter(source) => match source {
                InvalidDelimiterSource::Error(e) => e.to_string(),
                InvalidDelimiterSource::String(s) => format!("invalid delimiter found \"{}\"", s),
            },
        };
        write!(f, "failed to parse header, {}", msg)
    }
}
//}

//{ `record` module

pub struct ParseRecordError;
pub struct ParseRecordFilterError;
pub struct TryFromRawRecordError;
//}

//{ `csv` module

pub struct ReadCsvError;
pub struct WriteCsvError;
pub enum CsvError {
    ReadErr,
    WriteErr,
}
//}
