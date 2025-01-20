/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Types for storing measurement data records.

use crate::error::*;
use crate::format::CsvFormat;

use std::hash::{Hash, Hasher};

/// Stores a line of values as individual strings.
///
/// This is for textual processing only, used in intermediary operations, as soundness of the
/// values is not checked here thoroughly. For actually using the values in numeric form, see
/// `MetRecord`.
///
/// Fields `time` and `station_number` are mandatory, but all the others are wrapped in `Option`
/// to be able to represent missing values and filtered columns.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RawRecord {
    time: String,
    station_number: String,
    station_name: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    elevation: Option<String>,
    rain: Option<String>,
    q_rain: Option<String>,
    temp: Option<String>,
    q_temp: Option<String>,
    temp_avg: Option<String>,
    q_temp_avg: Option<String>,
    temp_min: Option<String>,
    q_temp_min: Option<String>,
    temp_max: Option<String>,
    q_temp_max: Option<String>,
    visibility: Option<String>,
    q_visibility: Option<String>,
    pressure: Option<String>,
    q_pressure: Option<String>,
    humidity: Option<String>,
    q_humidity: Option<String>,
    gamma_rad: Option<String>,
    q_gamma_rad: Option<String>,
    solar_rad: Option<String>,
    q_solar_rad: Option<String>,
    uv_rad: Option<String>,
    q_uv_rad: Option<String>,
    wind_speed: Option<String>,
    q_wind_speed: Option<String>,
    wind_dir: Option<String>,
    q_wind_dir: Option<String>,
    gust_speed: Option<String>,
    q_gust_speed: Option<String>,
    gust_dir: Option<String>,
    q_gust_dir: Option<String>,
    gust_minute: Option<String>,
    q_gust_minute: Option<String>,
    gust_second: Option<String>,
    q_gust_second: Option<String>,
    ground_temp_5: Option<String>,
    q_ground_temp_5: Option<String>,
    ground_temp_10: Option<String>,
    q_ground_temp_10: Option<String>,
    ground_temp_20: Option<String>,
    q_ground_temp_20: Option<String>,
    ground_temp_50: Option<String>,
    q_ground_temp_50: Option<String>,
    ground_temp_100: Option<String>,
    q_ground_temp_100: Option<String>,
    surface_temp: Option<String>,
    q_surface_temp: Option<String>,
    water_temp: Option<String>,
    q_water_temp: Option<String>,
    eor: Option<String>,
}
impl RawRecord {
    /// Tries to parse a CSV line using the specified format.
    pub fn from_csv(_line: &str, _format: CsvFormat) -> Result<Self, ParseRecordError> {
        todo!()
    }

    /// Creates a CSV line by concatenating the fields according to the specified format.
    pub fn to_csv(&self, _format: CsvFormat) -> String {
        todo!()
    }
}
impl Hash for RawRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.station_number.hash(state);
    }
}

/// Stores field values in their natural data types.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MetRecord {
    time: u64, //TODO chrono::DateTime
    station_number: usize,
    station_name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation: Option<f64>,
    rain: Option<f64>,
    q_rain: Option<String>,
    temp: Option<f64>,
    q_temp: Option<String>,
    temp_avg: Option<f64>,
    q_temp_avg: Option<String>,
    temp_min: Option<f64>,
    q_temp_min: Option<String>,
    temp_max: Option<f64>,
    q_temp_max: Option<String>,
    visibility: Option<f64>,
    q_visibility: Option<String>,
    pressure: Option<f64>,
    q_pressure: Option<String>,
    humidity: Option<f64>,
    q_humidity: Option<String>,
    gamma_rad: Option<f64>,
    q_gamma_rad: Option<String>,
    solar_rad: Option<f64>,
    q_solar_rad: Option<String>,
    uv_rad: Option<f64>,
    q_uv_rad: Option<String>,
    wind_speed: Option<f64>,
    q_wind_speed: Option<String>,
    wind_dir: Option<f64>,
    q_wind_dir: Option<String>,
    gust_speed: Option<f64>,
    q_gust_speed: Option<String>,
    gust_dir: Option<f64>,
    q_gust_dir: Option<String>,
    gust_minute: Option<f64>,
    q_gust_minute: Option<String>,
    gust_second: Option<f64>,
    q_gust_second: Option<String>,
    ground_temp_5: Option<f64>,
    q_ground_temp_5: Option<String>,
    ground_temp_10: Option<f64>,
    q_ground_temp_10: Option<String>,
    ground_temp_20: Option<f64>,
    q_ground_temp_20: Option<String>,
    ground_temp_50: Option<f64>,
    q_ground_temp_50: Option<String>,
    ground_temp_100: Option<f64>,
    q_ground_temp_100: Option<String>,
    surface_temp: Option<f64>,
    q_surface_temp: Option<String>,
    water_temp: Option<f64>,
    q_water_temp: Option<String>,
    eor: Option<String>,
}
impl Hash for MetRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.station_number.hash(state);
    }
}
impl TryFrom<RawRecord> for MetRecord {
    type Error = TryFromRawRecordError;

    /// String fields are moved, numeric fields are parsed.
    fn try_from(_value: RawRecord) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&RawRecord> for MetRecord {
    type Error = TryFromRawRecordError;

    /// String fields are copied, numeric fields are parsed.
    fn try_from(_value: &RawRecord) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// Stores a definition on how to filter individual records.
///
/// Only filtering by station is supported at this time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordFilter<'a> {
    name_set: Vec<&'a str>,
    number_set: Vec<&'a str>,
    exclude: bool,
}
impl<'a> RecordFilter<'a> {
    /// Tries to parse a comma-separated list of station numbers and names, supplied as a string
    /// (e.g. by the user).
    pub fn from_str(
        _filter_str: &'a str,
        _invert: bool,
    ) -> Result<Option<Self>, ParseRecordFilterError> {
        todo!()
    }

    /// Function to use with `Iterator.filter`.
    pub fn filter(_rec: &RawRecord) -> bool {
        todo!()
    }
}
