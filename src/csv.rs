/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Functions for manipulating CSV data.

use crate::error::*;
use crate::format::*;
use crate::record::*;

use std::io::{BufRead, Write};

/// Verifies if every line in the input is valid by trying to convert it to `MetRecord` (thus
/// looking for parsing errors).
///
/// If a `RecordFilter` is supplied, only basic checks are run for those lines that do not
/// satisfy the filter, and the conversion step will be skipped.
pub fn validate_csv(
    _reader: &impl BufRead,
    _record_filter: Option<&RecordFilter>,
) -> Result<(), ReadCsvError> {
    todo!()
}

/// Converts each line in the input to a `RawRecord`.
///
/// As an auxiliary output, returns the detected format definition as well.
pub fn read_csv<T>(
    _reader: &impl BufRead,
    _record_filter: Option<&RecordFilter>,
) -> (T, CsvFormat)
where
    T: Iterator<Item = Result<RawRecord, ReadCsvError>>,
{
    todo!()
}

/// Tries to write out the supplied records with the format specified.
pub fn write_csv(
    _records: impl IntoIterator<Item = RawRecord>,
    _writer: &impl Write,
    _format: &CsvFormat,
) -> Result<(), WriteCsvError> {
    todo!()
}

/// Convenience routine to combine `read_csv` and `write_csv`.
pub fn convert_csv(
    _reader: &impl BufRead,
    _writer: &impl Write,
    _record_filter: Option<&RecordFilter>,
    _format: &CsvFormat,
) -> Result<(), CsvError> {
    todo!()
}
