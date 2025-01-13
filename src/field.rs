/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Types representing CSV fields (i.e. columns) and related items.

use self::field_properties::*;
use crate::error::*;

use std::fmt;
use std::fmt::Display;

mod field_properties;

#[cfg(test)]
mod test;

/// Stores a list of selected CSV `Field`s.
///
/// For reading operations, this will represent fields that must be present in the input,
/// while for writing it will be used to apply column filtering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldConfig {
    fields: Vec<Field>,
}
impl FieldConfig {
    /// Constructs a default instance with all fields selected (i.e. no filter).
    pub fn new_with_all() -> Self {
        Self::new(true, true, true, true, None)
    }

    /// Constructs an instance by selecting field types and supplying a filter definition.
    ///
    /// Fields `Time` and `StationNumber` are mandatory and will always be included.
    ///
    /// Type categories for the other fields:
    /// - `info`: `StationName`, `Latitude`, `Longitude`, `Elevation`.
    /// - `values`: Columns with actual measurement data (i.e. everything else that is not a
    ///   Q-field or `EOR`).
    /// - `q`: For every value field there is a matching Q-field that is reserved for development
    ///   purposes by Met (and generally empty).
    /// - `eor`: "End Of Record" marker column at the end of every line, having the value `EOR`.
    pub fn new(
        info: bool,
        values: bool,
        q: bool,
        eor: bool,
        filter: Option<&FieldFilter>
    ) -> Self {
        Self {
            fields: FIELD_ARRAY
                .into_iter()
                .filter(|f| {
                    let pre = match f.field_type {
                        FieldType::Mandatory => true,
                        FieldType::Info => info,
                        FieldType::Value => values,
                        FieldType::Q => q,
                        FieldType::EOR => eor,
                    };
                    if let Some(filter) = filter {
                        match pre {
                            true => !filter.excluding.contains(&f.field),
                            false => filter.including.contains(&f.field),
                        }
                    } else {
                        pre
                    }
                })
                .map(|f| f.field)
                .collect(),
        }
    }

    /// Tries to parse a CSV header line to extract the fields used.
    pub fn from_header(header: &str) -> Result<Self, ParseHeaderError> {
        todo!()
    }
}
impl Default for FieldConfig {
    /// Calls `new_with_all`.
    fn default() -> Self {
        Self::new_with_all()
    }
}

/// Represents two sets, one to *include* and one to *exclude* fields in a `FieldConfig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldFilter {
    pub including: Vec<Field>,
    pub excluding: Vec<Field>,
}
impl FieldFilter {
    /// Constructs an instance by explicitly supplying the `Field`s to include and exclude.
    pub fn new<T>(
        include_set: T,
        exclude_set: T,
    ) -> Option<Self>
    where
        T: IntoIterator<Item = Field>,
    {
        let including: Vec<Field> = include_set.into_iter().collect();
        let excluding: Vec<Field> = exclude_set.into_iter().collect();
        if including.is_empty() && excluding.is_empty() {
            None
        } else {
            Some(FieldFilter {including, excluding})
        }
    }

    /// Tries to parse two comma-separated lists, supplied as strings (e.g. by the user).
    pub fn from_strs(
        include_str: Option<&str>,
        exclude_str: Option<&str>,
    ) -> Result<Option<Self>, ParseFieldFilterError> {
        todo!()
    }
}

/// All the columns in an unmodified source CSV downloaded from ODP,
/// these can be used as possible elements in a `FieldFilter`.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Field {
    Time,
    StationNumber,
    StationName,
    Latitude,
    Longitude,
    Elevation,
    Rain,
    Q_Rain,
    Temp,
    Q_Temp,
    TempAvg,
    Q_TempAvg,
    TempMin,
    Q_TempMin,
    TempMax,
    Q_TempMax,
    Visibility,
    Q_Visibility,
    Pressure,
    Q_Pressure,
    Humidity,
    Q_Humidity,
    GammaRad,
    Q_GammaRad,
    SolarRad,
    Q_SolarRad,
    UvRad,
    Q_UvRad,
    WindSpeed,
    Q_WindSpeed,
    WindDir,
    Q_WindDir,
    GustSpeed,
    Q_GustSpeed,
    GustDir,
    Q_GustDir,
    GustMinute,
    Q_GustMinute,
    GustSecond,
    Q_GustSecond,
    GroundTemp5,
    Q_GroundTemp5,
    GroundTemp10,
    Q_GroundTemp10,
    GroundTemp20,
    Q_GroundTemp20,
    GroundTemp50,
    Q_GroundTemp50,
    GroundTemp100,
    Q_GroundTemp100,
    SurfaceTemp,
    Q_SurfaceTemp,
    WaterTemp,
    Q_WaterTemp,
    EOR,
}
impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", FIELD_ARRAY[*self as usize].title)
    }
}
