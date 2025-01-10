/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use crate::error::*;
use crate::format::CsvFormat;

pub struct RawRecord {
    time: String,
    station_number: String,
    station_name: Option<String>,
    //...
}
impl RawRecord {
    fn from_csv(line: &str, format: CsvFormat) -> Result<Self, ParseRecordError> {
        todo!()
    }

    fn to_csv(&self, format: CsvFormat) -> String {
        todo!()
    }
}
//~ impl Display for RawRecord {}

pub struct MetRecord {
    time: u64, //TODO chrono::DateTime
    station_number: usize,
    station_name: Option<String>,
    //...
}
//~ impl TryFrom<RawRecord> for MetRecord {}

pub struct RecordFilter<'a> {
    name_set: Vec<&'a str>,
    number_set: Vec<&'a str>,
    exclude: bool,
}
impl<'a> RecordFilter<'a> {
    fn from_str(filter_str: &'a str) -> Result<Option<Self>, ParseRecordFilterError> {
        todo!()
    }

    fn filter(rec: &RawRecord) -> bool {
        todo!()
    }
}
