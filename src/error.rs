/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

pub struct ParseHeaderError;
pub struct ParseFieldFilterError;

pub struct ParseRecordError;
pub struct ParseRecordFilterError;

pub struct ReadCsvError;
pub struct WriteCsvError;
pub enum CsvError {
    ReadErr,
    WriteErr,
}
