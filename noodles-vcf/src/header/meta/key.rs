//! VCF header meta record key.

use std::{error, fmt, str::FromStr};

/// A VCF header meta record key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    /// (`ID`).
    Id,
    /// (`Type`).
    Type,
    /// (`Number`).
    Number,
    /// (`Values`).
    Values,
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        match self {
            Self::Id => "ID",
            Self::Type => "Type",
            Self::Number => "Number",
            Self::Values => "Values",
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// An error returned when a raw VCF header meta record key fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The input is invalid.
    Invalid,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::Invalid => f.write_str("invalid input"),
        }
    }
}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::Empty),
            "ID" => Ok(Self::Id),
            "Type" => Ok(Self::Type),
            "Number" => Ok(Self::Number),
            "Values" => Ok(Self::Values),
            _ => Err(ParseError::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(Key::Id.to_string(), "ID");
        assert_eq!(Key::Type.to_string(), "Type");
        assert_eq!(Key::Number.to_string(), "Number");
        assert_eq!(Key::Values.to_string(), "Values");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("ID".parse(), Ok(Key::Id));
        assert_eq!("Type".parse(), Ok(Key::Type));
        assert_eq!("Number".parse(), Ok(Key::Number));
        assert_eq!("Values".parse(), Ok(Key::Values));

        assert_eq!("".parse::<Key>(), Err(ParseError::Empty));
        assert_eq!("Noodles".parse::<Key>(), Err(ParseError::Invalid));
    }
}
