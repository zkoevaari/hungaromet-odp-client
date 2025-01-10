/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use crate::error::*;
use crate::record::*;

use std::io::{BufRead, Write};

fn validate_csv(
    reader: &impl BufRead,
    record_filter: Option<&RecordFilter>,
) -> Result<(), ReadCsvError> {
    todo!()
}

fn read_csv<T>(
    reader: &impl BufRead,
    record_filter: Option<&RecordFilter>,
) -> T
where T:  Iterator<Item = Result<RawRecord, ReadCsvError>>
{
    todo!()
}

fn write_csv(
    records: impl IntoIterator<Item = RawRecord>,
    writer: &impl Write,
) -> Result<(), WriteCsvError> {
    todo!()
}

fn convert_csv(
    reader: &impl BufRead,
    writer: &impl Write,
    record_filter: Option<&RecordFilter>,
) -> Result<(), CsvError> {
    todo!()
}
