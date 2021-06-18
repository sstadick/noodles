//! VCF record filters.

use std::{error, fmt, str::FromStr};

use indexmap::IndexSet;

use super::MISSING_FIELD;

const PASS_STATUS: &str = "PASS";
const DELIMITER: char = ';';

/// VCF record filters (`FILTER`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Filters {
    /// Missing (`.`).
    Missing,
    /// Pass (`PASS`).
    Pass,
    /// A list of filters that caused the record to fail.
    Fail(IndexSet<String>),
}

/// An error returned when raw VCF filters fail to convert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TryFromIteratorError {
    /// A filter is duplicated.
    DuplicateFilter(String),
    /// A filter is invalid.
    InvalidFilter(String),
}

impl error::Error for TryFromIteratorError {}

impl fmt::Display for TryFromIteratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateFilter(filter) => write!(f, "duplicate filter: {}", filter),
            Self::InvalidFilter(s) => write!(f, "invalid filter: {}", s),
        }
    }
}

impl Filters {
    /// Performs a conversion from a string iterator to a set of filters.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::record::Filters;
    ///
    /// let filters = Filters::try_from_iter(&["PASS"])?;
    /// assert_eq!(filters, Filters::Pass);
    ///
    /// let filters = Filters::try_from_iter(&["q10", "s50"])?;
    /// assert_eq!(filters, Filters::Fail(vec![
    ///     String::from("q10"),
    ///     String::from("s50"),
    /// ].into_iter().collect()));
    ///
    /// # Ok::<(), noodles_vcf::record::filters::TryFromIteratorError>(())
    /// ```
    pub fn try_from_iter<I, V>(iter: I) -> Result<Self, TryFromIteratorError>
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        let mut filters = IndexSet::new();

        for value in iter {
            let s = value.as_ref();

            if !filters.insert(s.into()) {
                return Err(TryFromIteratorError::DuplicateFilter(s.into()));
            } else if !is_valid_filter(s) {
                return Err(TryFromIteratorError::InvalidFilter(s.into()));
            }
        }

        if filters.is_empty() {
            Ok(Self::Missing)
        } else if filters.len() == 1 && filters.contains(PASS_STATUS) {
            Ok(Self::Pass)
        } else {
            Ok(Self::Fail(filters))
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::Missing
    }
}

impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => f.write_str(MISSING_FIELD),
            Self::Pass => f.write_str(PASS_STATUS),
            Self::Fail(ids) => {
                for (i, id) in ids.iter().enumerate() {
                    if i > 0 {
                        write!(f, "{}", DELIMITER)?;
                    }

                    f.write_str(id)?;
                }

                Ok(())
            }
        }
    }
}

/// An error returned when a raw VCF filter fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The filters are invalid.
    InvalidFilters(TryFromIteratorError),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::InvalidFilters(e) => write!(f, "invalid filters: {}", e),
        }
    }
}

impl FromStr for Filters {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::Empty),
            MISSING_FIELD => Ok(Self::Missing),
            PASS_STATUS => Ok(Self::Pass),
            _ => Self::try_from_iter(s.split(DELIMITER)).map_err(ParseError::InvalidFilters),
        }
    }
}

fn is_valid_filter(s: &str) -> bool {
    match s {
        "" | "0" => false,
        _ => s.chars().all(|c| !c.is_ascii_whitespace()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_iter() {
        assert_eq!(Filters::try_from_iter(&[] as &[&str]), Ok(Filters::Missing));
        assert_eq!(Filters::try_from_iter(&["PASS"]), Ok(Filters::Pass));
        assert_eq!(
            Filters::try_from_iter(&["q10"]),
            Ok(Filters::Fail(
                vec![String::from("q10")].into_iter().collect()
            ))
        );
        assert_eq!(
            Filters::try_from_iter(&["q10", "s50"]),
            Ok(Filters::Fail(
                vec![String::from("q10"), String::from("s50")]
                    .into_iter()
                    .collect()
            ))
        );

        assert_eq!(
            Filters::try_from_iter(&["q10", "q10"]),
            Err(TryFromIteratorError::DuplicateFilter(String::from("q10")))
        );
        assert_eq!(
            Filters::try_from_iter(&[""]),
            Err(TryFromIteratorError::InvalidFilter(String::from("")))
        );
        assert_eq!(
            Filters::try_from_iter(&["0"]),
            Err(TryFromIteratorError::InvalidFilter(String::from("0")))
        );
        assert_eq!(
            Filters::try_from_iter(&["q 10"]),
            Err(TryFromIteratorError::InvalidFilter(String::from("q 10")))
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Filters::default(), Filters::Missing);
    }

    #[test]
    fn test_fmt() -> Result<(), TryFromIteratorError> {
        assert_eq!(Filters::Missing.to_string(), ".");
        assert_eq!(Filters::Pass.to_string(), "PASS");

        let filters = Filters::try_from_iter(&["q10"])?;
        assert_eq!(filters.to_string(), "q10");

        let filters = Filters::try_from_iter(&["q10", "s50"])?;
        assert_eq!(filters.to_string(), "q10;s50");

        Ok(())
    }

    #[test]
    fn test_from_str() {
        assert_eq!(".".parse(), Ok(Filters::Missing));
        assert_eq!("PASS".parse(), Ok(Filters::Pass));

        assert_eq!(
            "q10".parse(),
            Ok(Filters::Fail(
                vec![String::from("q10")].into_iter().collect()
            ))
        );

        assert_eq!(
            "q10;s50".parse(),
            Ok(Filters::Fail(
                vec![String::from("q10"), String::from("s50")]
                    .into_iter()
                    .collect()
            ))
        );

        assert_eq!("".parse::<Filters>(), Err(ParseError::Empty));
        assert_eq!(
            "q10;q10".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::DuplicateFilter(String::from("q10"))
            ))
        );
        assert_eq!(
            "0".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::InvalidFilter(String::from("0"))
            ))
        );
        assert_eq!(
            "q 10".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::InvalidFilter(String::from("q 10"))
            ))
        );
        assert_eq!(
            ";q10".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::InvalidFilter(String::from(""))
            ))
        );
        assert_eq!(
            "q10;;s50".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::InvalidFilter(String::from(""))
            ))
        );
        assert_eq!(
            "q10;".parse::<Filters>(),
            Err(ParseError::InvalidFilters(
                TryFromIteratorError::InvalidFilter(String::from(""))
            ))
        );
    }
}
