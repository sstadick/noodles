//! SAM CIGAR and operations.

pub mod op;

use std::{error, fmt, ops::Deref, str::FromStr};

pub use self::op::Op;

use super::NULL_FIELD;

use self::op::Kind;

/// A SAM record CIGAR.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cigar(Vec<Op>);

impl Cigar {
    /// Calculates the alignment span over the reference sequence.
    ///
    /// This sums the lengths of the CIGAR operations that consume the reference sequence, i.e.,
    /// alignment matches (`M`), deletions from the reference (`D`), skipped reference regions
    /// (`S`), sequence matches (`=`), and sequence mismatches (`X`).
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::record::{cigar::{op::Kind, Op}, Cigar};
    ///
    /// let cigar = Cigar::from(vec![
    ///     Op::new(Kind::Match, 36),
    ///     Op::new(Kind::Deletion, 4),
    ///     Op::new(Kind::SoftClip, 8),
    /// ]);
    ///
    /// assert_eq!(cigar.reference_len(), 40);
    /// ```
    pub fn reference_len(&self) -> u32 {
        self.iter()
            .filter_map(|op| match op.kind() {
                Kind::Match | Kind::Deletion | Kind::Skip | Kind::SeqMatch | Kind::SeqMismatch => {
                    Some(op.len())
                }
                _ => None,
            })
            .sum()
    }
}

impl Deref for Cigar {
    type Target = [Op];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Cigar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "{}", NULL_FIELD)
        } else {
            for op in self.iter() {
                write!(f, "{}", op)?;
            }

            Ok(())
        }
    }
}

impl From<Vec<Op>> for Cigar {
    fn from(ops: Vec<Op>) -> Self {
        Self(ops)
    }
}

/// An error returned when a raw CIGAR string fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The input is invalid.
    Invalid,
    /// The CIGAR string has an invalid operation.
    InvalidOp(op::ParseError),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::Invalid => f.write_str("invalid input"),
            Self::InvalidOp(e) => write!(f, "invalid op: {}", e),
        }
    }
}

impl FromStr for Cigar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        } else if s == NULL_FIELD {
            return Ok(Self::default());
        }

        let mut ops = Vec::new();

        let matches = s.match_indices(|c: char| !c.is_digit(10));
        let mut start = 0;

        for (end, raw_kind) in matches {
            let op = s[start..=end].parse().map_err(ParseError::InvalidOp)?;
            ops.push(op);
            start = end + raw_kind.len();
        }

        if start == s.len() {
            Ok(Self::from(ops))
        } else {
            Err(ParseError::Invalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let cigar = Cigar::default();
        assert!(cigar.is_empty());

        let cigar = Cigar::from(vec![Op::new(Kind::Match, 1)]);
        assert!(!cigar.is_empty());
    }

    #[test]
    fn test_fmt() {
        let cigar = Cigar::from(vec![
            Op::new(Kind::Match, 1),
            Op::new(Kind::Skip, 13),
            Op::new(Kind::SoftClip, 144),
        ]);

        assert_eq!(cigar.to_string(), "1M13N144S");
    }

    #[test]
    fn test_fmt_when_cigar_has_no_ops() {
        let cigar = Cigar::default();
        assert_eq!(cigar.to_string(), "*");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            "1M13N144S".parse(),
            Ok(Cigar::from(vec![
                Op::new(Kind::Match, 1),
                Op::new(Kind::Skip, 13),
                Op::new(Kind::SoftClip, 144),
            ]))
        );

        assert_eq!("*".parse(), Ok(Cigar::default()));

        assert_eq!("".parse::<Cigar>(), Err(ParseError::Empty));
        assert_eq!("8M13".parse::<Cigar>(), Err(ParseError::Invalid));
    }
}
