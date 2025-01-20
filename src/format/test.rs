/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use super::*;

use crate::field::test::TEST_HEADER;

// Checking the `Delimiter` constructor.
#[test]
fn test_delimiter() {
    for ch in [';', ',', ' ', '\t', '|', '#'] {
        assert_eq!(Delimiter::try_from(ch).unwrap().0, ch);
    }

    for ch in ['0', 'X', '\0', '\x1f'] {
        assert_eq!(Delimiter::try_from(ch).unwrap_err().0, ch);
    }
}

// Checking the `MissingValue` constructor.
#[test]
fn test_missing() {
    for s in ["-999", "null", "Null", "NULL", ""] {
        let res = MissingValue::try_from(s);
        assert!(matches!(res, Ok(_)));
    }

    for s in ["999", "nil", "Nyull", "NUL", " ", "\0"] {
        assert_eq!(MissingValue::try_from(s).unwrap_err().0, s);
    }
}

// Checking `CsvFormat` parsing and display.
#[test]
fn test_csv_format() {
    let format = CsvFormat::from_str(TEST_HEADER).unwrap();
    assert_eq!(format, CsvFormat::default());
    assert_eq!(format.to_string(), TEST_HEADER);

    let ffilter = FieldFilter::new([Field::Temp], std::iter::empty());
    let fconfig = FieldConfig::new(false, false, false, true, ffilter.as_ref());

    for (header, alignment, delim, missing) in [
        ("        Time,StationNumber,    t,EOR", true, ',', MissingValue::Minus999),
        ("Time;StationNumber;t;EOR", false, ';', MissingValue::Empty),
        ("Time,StationNumber,t,EOR", false, ',', MissingValue::Empty),
        ("Time StationNumber t EOR", false, ' ', MissingValue::Null),
    ] {
        let expected = CsvFormat {
            alignment,
            delimiter: Delimiter::try_from(delim).unwrap(),
            missing,
            field_config: fconfig.clone(),
        };
        let format = CsvFormat::from_str(header).unwrap();
        assert_eq!(format, expected);
        assert_eq!(format.to_string(), header);
    }

    for (header, eexpected) in [
        (
            "StationNumber;t;EOR",
            ParseHeaderError::MissingTime,
        ),
        (
            "         Time;StationNumber;t;EOR",
            ParseHeaderError::InvalidTimeAlignment,
        ),
        (
            "________Time;StationNumber;t;EOR",
            ParseHeaderError::InvalidTimeAlignment,
        ),
        (
            "Date;Time;StationNumber;t;EOR",
            ParseHeaderError::InvalidTimeAlignment,
        ),
        (
            "Time;StationName;t;EOR",
            ParseHeaderError::MissingStationNumber,
        ),
        (
            "Time         ;StationNumber;t;EOR",
            ParseHeaderError::InvalidDelimiter(InvalidDelimiterSource::String(
                "         ;".to_string(),
            )),
        ),
        (
            "Time;StationName;StationNumber;t;EOR",
            ParseHeaderError::InvalidDelimiter(InvalidDelimiterSource::String(
                ";StationName;".to_string(),
            )),
        ),
        (
            "Time\x1fStationNumber\x1ft\x1fEOR",
            ParseHeaderError::InvalidDelimiter(InvalidDelimiterSource::Error(
                InvalidDelimiterError('\x1f'),
            )),
        ),
        // Finally to check if errors come through from `FieldConfig`:
        (
            "Time;StationNumber;EOR;t",
            ParseHeaderError::InvalidEorPosition,
        ),
    ] {
        assert_eq!(CsvFormat::from_str(header).unwrap_err(), eexpected);
    }
}
