/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use crate::error::*;

pub struct FieldConfig {
    fields: Vec<Field>,
}
impl FieldConfig {
    pub fn new_with_all() -> Self {
        todo!()
    }

    pub fn new(
        info: bool,
        values: bool,
        q: bool,
        eor: bool,
        filter: Option<&FieldFilter>
    ) -> Self {
        todo!()
    }

    pub fn from_header(header: &str) -> Result<Self, ParseHeaderError> {
        todo!()
    }
}

pub struct FieldFilter {
    including: Vec<Field>,
    excluding: Vec<Field>,
}
impl FieldFilter {
    pub fn from_strs(
        include_str: Option<&str>,
        exclude_str: Option<&str>,
    ) -> Result<Option<Self>, ParseFieldFilterError> {
        todo!()
    }

    pub fn new<T>(
        include_set: T,
        exclude_set: T,
    ) -> Option<Self>
    where T: IntoIterator<Item = Field>,
    {
        todo!()
    }

    pub fn include_list(&self) -> &[Field] {
        todo!()
    }

    pub fn exclude_list(&self) -> &[Field] {
        todo!()
    }
}

pub enum Field {
    Time = 1,
    StationNumber,
    StationName,
    //...
}

struct FieldProperties {
    title: &'static str,
    field: Field,
    field_type: FieldType,
    width: u8,
}

pub enum FieldType {
    Mandatory,
    Info,
    Value,
    Q,
    EOR,
}

const FIELD_ARRAY: [FieldProperties; 3] = [
    FieldProperties {
        title: "Time",
        field: Field::Time,
        field_type: FieldType::Mandatory,
        width: 12,
    },
    FieldProperties {
        title: "StationNumber",
        field: Field::StationNumber,
        field_type: FieldType::Mandatory,
        width: 13,
    },
    FieldProperties {
        title: "StationName",
        field: Field::StationName,
        field_type: FieldType::Info,
        width: 40,
    },
    //...
];
