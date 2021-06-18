//! SAM header read group tag.

use std::{error, fmt, str::FromStr};

/// A SAM header read group tag.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Tag {
    /// Read group ID (`ID`).
    Id,
    /// Barcode sequence (`BC`).
    Barcode,
    /// Sequencing center (`CN`).
    SequencingCenter,
    /// Description (`DS`).
    Description,
    /// Datetime of run (`DT`).
    ProducedAt,
    /// Flow order (`FO`).
    FlowOrder,
    /// Key sequence (`KS`).
    KeySequence,
    /// Library (`LB`).
    Library,
    /// Programs used (`PG`).
    Program,
    /// Predicted median insert size (`PI`).
    PredictedMedianInsertSize,
    /// Platform used (`PL`).
    Platform,
    /// Platform model (`PM`).
    PlatformModel,
    /// Platform unit (`PU`).
    PlatformUnit,
    /// Sample (`SM`).
    Sample,
    /// Any other read group tag.
    Other(String),
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        match self {
            Self::Id => "ID",
            Self::Barcode => "BC",
            Self::SequencingCenter => "CN",
            Self::Description => "DS",
            Self::ProducedAt => "DT",
            Self::FlowOrder => "FO",
            Self::KeySequence => "KS",
            Self::Library => "LB",
            Self::Program => "PG",
            Self::PredictedMedianInsertSize => "PI",
            Self::Platform => "PL",
            Self::PlatformModel => "PM",
            Self::PlatformUnit => "PU",
            Self::Sample => "SM",
            Self::Other(s) => s,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// An error returned when a raw SAM header read group tag fails to parse.
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

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::Empty),
            "ID" => Ok(Self::Id),
            "BC" => Ok(Self::Barcode),
            "CN" => Ok(Self::SequencingCenter),
            "DS" => Ok(Self::Description),
            "DT" => Ok(Self::ProducedAt),
            "FO" => Ok(Self::FlowOrder),
            "KS" => Ok(Self::KeySequence),
            "LB" => Ok(Self::Library),
            "PG" => Ok(Self::Program),
            "PI" => Ok(Self::PredictedMedianInsertSize),
            "PL" => Ok(Self::Platform),
            "PM" => Ok(Self::PlatformModel),
            "PU" => Ok(Self::PlatformUnit),
            "SM" => Ok(Self::Sample),
            _ => {
                if s.len() == 2 {
                    Ok(Self::Other(s.into()))
                } else {
                    Err(ParseError::Invalid)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(Tag::Id.to_string(), "ID");
        assert_eq!(Tag::Barcode.to_string(), "BC");
        assert_eq!(Tag::SequencingCenter.to_string(), "CN");
        assert_eq!(Tag::Description.to_string(), "DS");
        assert_eq!(Tag::ProducedAt.to_string(), "DT");
        assert_eq!(Tag::FlowOrder.to_string(), "FO");
        assert_eq!(Tag::KeySequence.to_string(), "KS");
        assert_eq!(Tag::Library.to_string(), "LB");
        assert_eq!(Tag::Program.to_string(), "PG");
        assert_eq!(Tag::PredictedMedianInsertSize.to_string(), "PI");
        assert_eq!(Tag::Platform.to_string(), "PL");
        assert_eq!(Tag::PlatformModel.to_string(), "PM");
        assert_eq!(Tag::PlatformUnit.to_string(), "PU");
        assert_eq!(Tag::Sample.to_string(), "SM");
        assert_eq!(Tag::Other(String::from("ND")).to_string(), "ND");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("ID".parse(), Ok(Tag::Id));
        assert_eq!("BC".parse(), Ok(Tag::Barcode));
        assert_eq!("CN".parse(), Ok(Tag::SequencingCenter));
        assert_eq!("DS".parse(), Ok(Tag::Description));
        assert_eq!("DT".parse(), Ok(Tag::ProducedAt));
        assert_eq!("FO".parse(), Ok(Tag::FlowOrder));
        assert_eq!("KS".parse(), Ok(Tag::KeySequence));
        assert_eq!("LB".parse(), Ok(Tag::Library));
        assert_eq!("PG".parse(), Ok(Tag::Program));
        assert_eq!("PI".parse(), Ok(Tag::PredictedMedianInsertSize));
        assert_eq!("PL".parse(), Ok(Tag::Platform));
        assert_eq!("PM".parse(), Ok(Tag::PlatformModel));
        assert_eq!("PU".parse(), Ok(Tag::PlatformUnit));
        assert_eq!("SM".parse(), Ok(Tag::Sample));
        assert_eq!("ND".parse(), Ok(Tag::Other(String::from("ND"))));

        assert_eq!("".parse::<Tag>(), Err(ParseError::Empty));
        assert_eq!("NDL".parse::<Tag>(), Err(ParseError::Invalid));
    }
}
