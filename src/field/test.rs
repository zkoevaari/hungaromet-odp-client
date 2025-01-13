/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use super::*;

const TEST_HEADER: &str = "        Time;StationNumber;StationName                             ;Latitude;Longitude;Elevation;    r; Q_r;    t; Q_t;   ta;Q_ta;   tn;Q_tn;   tx;Q_tx;     v; Q_v;      p; Q_p;   u; Q_u;      sg;Q_sg;     sr;Q_sr;   suv;Q_suv;   fs;Q_fs; fsd;Q_fsd;   fx;Q_fx; fxd;Q_fxd; fxm;Q_fxm; fxs;Q_fxs;  et5;Q_et5; et10;Q_et10; et20;Q_et20; et50;Q_et50;et100;Q_et100;  tsn;Q_tsn; tviz;Q_tviz;EOR";

// Tests to check if `Field` and `FIELD_ARRAY` are in sync, and no errors were made during
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
}
