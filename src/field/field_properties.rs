/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Helper items for the `field` module.

use super::Field;

/// Auxiliary characteristics of fields, used in `FIELD_ARRAY`.
pub struct FieldProperties {
    pub title: &'static str,
    pub field: Field,
    pub field_type: FieldType,
    pub width: u8,
}

/// Categories of fields, used in filtering.
pub enum FieldType {
    Mandatory,
    Info,
    Value,
    Q,
    EOR,
}

/// Array used internally to link index and additional properties to `Field`s.
///
/// There must be a 1:1 association to the `Field` enum, in matching order
/// (i.e. array index == enum discriminant).
pub const FIELD_ARRAY: [FieldProperties; 55] = [
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
    FieldProperties {
        title: "Latitude",
        field: Field::Latitude,
        field_type: FieldType::Info,
        width: 8,
    },
    FieldProperties {
        title: "Longitude",
        field: Field::Longitude,
        field_type: FieldType::Info,
        width: 9,
    },
    FieldProperties {
        title: "Elevation",
        field: Field::Elevation,
        field_type: FieldType::Info,
        width: 9,
    },
    FieldProperties {
        title: "r",
        field: Field::Rain,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_r",
        field: Field::Q_Rain,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "t",
        field: Field::Temp,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_t",
        field: Field::Q_Temp,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "ta",
        field: Field::TempAvg,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_ta",
        field: Field::Q_TempAvg,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "tn",
        field: Field::TempMin,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_tn",
        field: Field::Q_TempMin,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "tx",
        field: Field::TempMax,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_tx",
        field: Field::Q_TempMax,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "v",
        field: Field::Visibility,
        field_type: FieldType::Value,
        width: 6,
    },
    FieldProperties {
        title: "Q_v",
        field: Field::Q_Visibility,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "p",
        field: Field::Pressure,
        field_type: FieldType::Value,
        width: 7,
    },
    FieldProperties {
        title: "Q_p",
        field: Field::Q_Pressure,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "u",
        field: Field::Humidity,
        field_type: FieldType::Value,
        width: 4,
    },
    FieldProperties {
        title: "Q_u",
        field: Field::Q_Humidity,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "sg",
        field: Field::GammaRad,
        field_type: FieldType::Value,
        width: 8,
    },
    FieldProperties {
        title: "Q_sg",
        field: Field::Q_GammaRad,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "sr",
        field: Field::SolarRad,
        field_type: FieldType::Value,
        width: 7,
    },
    FieldProperties {
        title: "Q_sr",
        field: Field::Q_SolarRad,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "suv",
        field: Field::UvRad,
        field_type: FieldType::Value,
        width: 6,
    },
    FieldProperties {
        title: "Q_suv",
        field: Field::Q_UvRad,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "fs",
        field: Field::WindSpeed,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_fs",
        field: Field::Q_WindSpeed,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "fsd",
        field: Field::WindDir,
        field_type: FieldType::Value,
        width: 4,
    },
    FieldProperties {
        title: "Q_fsd",
        field: Field::Q_WindDir,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "fx",
        field: Field::GustSpeed,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_fx",
        field: Field::Q_GustSpeed,
        field_type: FieldType::Q,
        width: 4,
    },
    FieldProperties {
        title: "fxd",
        field: Field::GustDir,
        field_type: FieldType::Value,
        width: 4,
    },
    FieldProperties {
        title: "Q_fxd",
        field: Field::Q_GustDir,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "fxm",
        field: Field::GustMinute,
        field_type: FieldType::Value,
        width: 4,
    },
    FieldProperties {
        title: "Q_fxm",
        field: Field::Q_GustMinute,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "fxs",
        field: Field::GustSecond,
        field_type: FieldType::Value,
        width: 4,
    },
    FieldProperties {
        title: "Q_fxs",
        field: Field::Q_GustSecond,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "et5",
        field: Field::GroundTemp5,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_et5",
        field: Field::Q_GroundTemp5,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "et10",
        field: Field::GroundTemp10,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_et10",
        field: Field::Q_GroundTemp10,
        field_type: FieldType::Q,
        width: 6,
    },
    FieldProperties {
        title: "et20",
        field: Field::GroundTemp20,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_et20",
        field: Field::Q_GroundTemp20,
        field_type: FieldType::Q,
        width: 6,
    },
    FieldProperties {
        title: "et50",
        field: Field::GroundTemp50,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_et50",
        field: Field::Q_GroundTemp50,
        field_type: FieldType::Q,
        width: 6,
    },
    FieldProperties {
        title: "et100",
        field: Field::GroundTemp100,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_et100",
        field: Field::Q_GroundTemp100,
        field_type: FieldType::Q,
        width: 7,
    },
    FieldProperties {
        title: "tsn",
        field: Field::SurfaceTemp,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_tsn",
        field: Field::Q_SurfaceTemp,
        field_type: FieldType::Q,
        width: 5,
    },
    FieldProperties {
        title: "tviz",
        field: Field::WaterTemp,
        field_type: FieldType::Value,
        width: 5,
    },
    FieldProperties {
        title: "Q_tviz",
        field: Field::Q_WaterTemp,
        field_type: FieldType::Q,
        width: 6,
    },
    FieldProperties {
        title: "EOR",
        field: Field::EOR,
        field_type: FieldType::EOR,
        width: 3,
    },
];
