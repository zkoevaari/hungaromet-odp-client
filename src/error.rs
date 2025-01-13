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

pub struct ParseHeaderError;
pub struct ParseFieldFilterError;

/// Error type used in `Delimiter` constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDelimiterError;
impl Error for InvalidDelimiterError {}
impl Display for InvalidDelimiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot construct delimiter, invalid character supplied")
    }
}

pub struct TryFromRawRecordError;
pub struct ParseRecordError;
pub struct ParseRecordFilterError;

pub struct ReadCsvError;
pub struct WriteCsvError;
pub enum CsvError {
    ReadErr,
    WriteErr,
}
