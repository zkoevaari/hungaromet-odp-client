/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Types related to CSV formatting configuration.

use crate::error::InvalidDelimiterError;
use crate::field::FieldConfig;

use std::fmt;
use std::fmt::Display;

/// How to represent a missing value in CSV.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MissingValue {
    /// "-999", this is the default.
    Minus999,
    /// "null"
    Null,
    /// ""
    Empty,
}
impl Default for MissingValue {
    /// Returns the variant `Minus999`.
    fn default() -> Self {
        Self::Minus999
    }
}
impl Display for MissingValue {
    /// Prints the string representation, see above (TODO).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Minus999 => "-999",
                Self::Null => "null",
                Self::Empty => "",
            }
        )
    }
}

/// Field separator character.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delimiter(char);
impl Delimiter {
    /// Constructor ensuring that the character is valid.
    pub fn new(ch: char) -> Result<Self, InvalidDelimiterError> {
        if ch.is_ascii_punctuation() || ch.is_ascii_whitespace() {
            Ok(Delimiter(ch))
        } else {
            Err(InvalidDelimiterError {})
        }
    }
}
impl Default for Delimiter {
    /// Semicolon (';').
    fn default() -> Self {
        Delimiter(';')
    }
}
impl Display for Delimiter {
    /// Prints the character.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// CSV formatting properties including the selected fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CsvFormat {
    /// - `true`: The field will be padded with spaces to the default width and the value aligned
    ///   within to the right (except for `StationName` which is left-aligned).
    /// - `false`: The field will be condensed to the width needed for the value.
    pub alignment: bool,
    pub missing: MissingValue,
    pub delimiter: Delimiter,
    pub field_config: FieldConfig,
}
impl Default for CsvFormat {
    /// Constructs the format that is used in the ODP data files.
    fn default() -> Self {
        CsvFormat {
            alignment: true,
            missing: MissingValue::default(),
            delimiter: Delimiter::default(),
            field_config: FieldConfig::default(),
        }
    }
}
impl Display for CsvFormat {
    /// Prints selected field titles separated by the delimiter, useful as a header line.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//~         write!(f, "", )
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_delimiter() {
        for ch in [';', ',', ' ', '\t', '|'] {
            let d = Delimiter::new(ch);
            assert!(matches!(d, Ok(_)));
            assert_eq!(d.unwrap().0, ch);
        }
        assert!(matches!(Delimiter::new(0.into()), Err(_)));
    }
}
