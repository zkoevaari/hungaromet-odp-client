/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use super::*;

pub(crate) const TEST_HEADER: &str = "        Time;StationNumber;StationName                             ;Latitude;Longitude;Elevation;    r; Q_r;    t; Q_t;   ta;Q_ta;   tn;Q_tn;   tx;Q_tx;     v; Q_v;      p; Q_p;   u; Q_u;      sg;Q_sg;     sr;Q_sr;   suv;Q_suv;   fs;Q_fs; fsd;Q_fsd;   fx;Q_fx; fxd;Q_fxd; fxm;Q_fxm; fxs;Q_fxs;  et5;Q_et5; et10;Q_et10; et20;Q_et20; et50;Q_et50;et100;Q_et100;  tsn;Q_tsn; tviz;Q_tviz;EOR";

// Checking if `Field` and `FIELD_ARRAY` are in sync, and no errors were made during
// copy-pasting (or later editing) the field definitions.
#[test]
fn test_field_array() {
    // Array index equals enum discriminant
    for (i, f) in FIELD_ARRAY.into_iter().enumerate() {
        assert_eq!(i, f.field as usize);
    }

    // EOR is last
    let last = &FIELD_ARRAY[FIELD_ARRAY.len() - 1];
    assert!(matches!(last.field, Field::EOR));
    assert!(matches!(last.field_type, FieldType::EOR));

    // Only one EOR is present
    let count = FIELD_ARRAY
        .into_iter()
        .filter(|f| matches!(f.field, Field::EOR) || matches!(f.field_type, FieldType::EOR))
        .count();
    assert_eq!(count, 1);

    // Width for all fields and delimiters equals length of the reference
    let test_len = TEST_HEADER.len();
    let calc_len = FIELD_ARRAY
        .into_iter()
        .map(|f| f.width)
        .fold(0_usize, |acc, w| acc + (w as usize) + 1);
    assert_eq!(calc_len, test_len + 1);

    // Fields starting with Q are exactly those that have FieldType::Q
    for (i, f) in FIELD_ARRAY.into_iter().enumerate() {
        let start = f.title.starts_with("Q_");
        let q = matches!(f.field_type, FieldType::Q);
        assert_eq!(start, q, "Q mismatch at index {i}");
    }

    // There are no duplicate titles
    for i in 0..(FIELD_ARRAY.len() - 1) {
        let t1 = FIELD_ARRAY[i].title;
        for j in (i + 1)..FIELD_ARRAY.len() {
            let t2 = FIELD_ARRAY[j].title;
            assert_ne!(t1, t2);
        }
    }
}

// Checking the string to `Field` conversion.
#[test]
fn test_field_try_from() {
    for s in [
        "Time",
        "StationNumber",
        "StationName",
        "t",
        "EOR",
    ] {
        assert_eq!(Field::try_from(s).unwrap().title(), s);
    }

    for s in [
        "time",
        "Station_Number",
        "Station Name",
        "T",
        "Eor",
        "eor",
        " EOR",
        "EOR ",
    ] {
        assert_eq!(Field::try_from(s).unwrap_err().0, s);
    }
}

// Checking `FieldConfig` parsing from string.
use crate::error::ParseHeaderError::*;
#[test]
fn test_fieldconfig() {
    let config = FieldConfig::from_header(TEST_HEADER, Delimiter::default()).unwrap();
    assert_eq!(config, FieldConfig::new_with_all());

    let header = "Time;StationNumber;EOR";
    let expected = FieldConfig::new(false, false, false, true, None);
    let config = FieldConfig::from_header(header, Delimiter::default()).unwrap();
    assert_eq!(config, expected);

    let header = "Time;StationNumber;t";
    let ffilter = FieldFilter::new([Field::Temp], std::iter::empty());
    let expected = FieldConfig::new(false, false, false, false, ffilter.as_ref());
    let config = FieldConfig::from_header(header, Delimiter::default()).unwrap();
    assert_eq!(config, expected);

    // TODO: add more `FieldConfig::new` variants, while experimenting with code coverage analysis

    for (header, eexpected) in [
        ("Time,StationNumber,t,EOR", FoundNoDelimiters),
        ("Time;StationNumber;Time;EOR", FoundDuplicates("Time".to_string())),
        ("Time;StationNumber;t;Eor", UnknownField(ParseFieldError("Eor".to_string()))),
        ("Time;StationNumber;t;EOR;", UnknownField(ParseFieldError(String::new()))),
        ("StationNumber;t;EOR", MissingTime),
        ("Time;StationName;t;EOR", MissingStationNumber),
        ("Time;StationNumber;EOR;t", InvalidEorPosition),
    ] {
        assert_eq!(
            FieldConfig::from_header(header, Delimiter::default()).unwrap_err(),
            eexpected,
        );
    }
}
