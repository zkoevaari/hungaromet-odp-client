/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use crate::field::FieldConfig;

pub enum Alignment {
    Right,
    None,
}

pub enum MissingValue {
    MinValue,
    Null,
    Empty,
}

struct Delimiter(char);

pub struct CsvFormat {
    alignment: Alignment,
    missing: MissingValue,
    delimiter: Delimiter,
    field_config: FieldConfig,
}
