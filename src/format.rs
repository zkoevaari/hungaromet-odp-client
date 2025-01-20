/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

//! Types related to CSV formatting configuration.

use crate::error::*;
use crate::field::*;

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(test)]
mod test;

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
        write!(f, "{}", match self {
            Self::Minus999 => "-999",
            Self::Null => "null",
            Self::Empty => "",
        })
    }
}
impl TryFrom<&str> for MissingValue {
    type Error = ParseMissingError;

    /// Constructs an instance from a field value given as string.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "-999" => Ok(Self::Minus999),
            "null" | "NULL" | "Null" => Ok(Self::Null),
            "" => Ok(Self::Empty),
            _ => Err(ParseMissingError::new(s)),
        }
    }
}

/// Field separator character.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delimiter(char);
impl AsRef<char> for Delimiter {
    fn as_ref(&self) -> &char {
        &self.0
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
impl TryFrom<char> for Delimiter {
    type Error = InvalidDelimiterError;

    /// Conversion ensuring that the character is valid.
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if ch.is_ascii_punctuation() || ch.is_ascii_whitespace() {
            Ok(Delimiter(ch))
        } else {
            Err(InvalidDelimiterError::new(ch))
        }
    }
}

/// CSV formatting properties including the selected fields.
///
/// It is used mainly to define the output CSV, but also helps in checking that every input line
/// conforms to the same format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CsvFormat {
    /// - `true`: The field will be padded with spaces to the default width and the value aligned
    ///   within to the right (except for `StationName` which is left-aligned).
    /// - `false`: The field will be condensed to the width needed for the value.
    pub alignment: bool,
    /// Indicates the desired behavior when the format is used for output. It is not intended to
    /// be used while checking input lines, as its initial setting cannot be derived from the CSV
    /// header (see `from_str`).
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
    /// Prints selected field titles separated by the delimiter, can be used as a header line.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, f) in self.field_config.fields().enumerate() {
            if i != 0 {
                write!(fmt, "{}", self.delimiter)?;
            }

            let title = f.title();
            let padding = usize::from(f.width()).saturating_sub(title.len());
            if self.alignment && *f != Field::StationName {
                for _ in 0..padding {
                    write!(fmt, " ")?;
                }
            }
            write!(fmt, "{}", title)?;
            if self.alignment && *f == Field::StationName {
                for _ in 0..padding {
                    write!(fmt, " ")?;
                }
            }
        }

        Ok(())
    }
}
impl FromStr for CsvFormat {
    type Err = ParseHeaderError;

    /// Tries to parse a CSV header string to extract the format.
    ///
    /// The `alignment` will be judged by the first column, `Time`. The character between `Time`
    /// and `StationNumber` will determine the delimiter.
    ///
    /// As the value used for `missing` cannot be reliably determined from the header, the
    /// following heuristic is used:
    /// - If alignment is used, it will be `Minus999` (conforming with the default).
    /// - Else if delimiter is space or tab, it will be `Null`.
    /// - Else `Empty`.
    ///
    /// This is just to have something sensible for `missing` as well, but in general the setting
    /// is not meant to be enforced when checking input validity. When the `CsvFormat` is used
    /// for output, `missing` should be explicitly set.
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        let time_title = Field::Time.title();
        let time_index = header
            .find(time_title)
            .ok_or(ParseHeaderError::MissingTime)?;

        let alignment = if time_index == 0 {
            false
        } else {
            if time_index > (Field::Time.width() as usize) - time_title.len()
                || header[..time_index]
                    .chars()
                    .any(|ch| !ch.is_ascii_whitespace())
            {
                return Err(ParseHeaderError::InvalidTimeAlignment);
            }
            true
        };

        let number_index = header
            .find(Field::StationNumber.title())
            .ok_or(ParseHeaderError::MissingStationNumber)?;

        let delim_index = time_index + time_title.len();
        if number_index > delim_index + 1 {
            return Err(ParseHeaderError::InvalidDelimiter(
                InvalidDelimiterSource::String(header[delim_index..number_index].to_string()),
            ));
        }
        let delimiter = match Delimiter::try_from(header.chars().nth(delim_index).unwrap()) {
            Ok(d) => d,
            Err(e) => {
                return Err(ParseHeaderError::InvalidDelimiter(
                    InvalidDelimiterSource::Error(e),
                ));
            }
        };

        let missing = if alignment {
            MissingValue::Minus999
        } else if delimiter.as_ref().is_ascii_whitespace() {
            MissingValue::Null
        } else {
            MissingValue::Empty
        };

        let field_config = FieldConfig::from_header(header, delimiter)?;

        Ok(CsvFormat {
            alignment,
            delimiter,
            missing,
            field_config,
        })
    }
}
