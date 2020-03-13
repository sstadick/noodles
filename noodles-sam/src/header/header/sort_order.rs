use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortOrder {
    Unknown,
    Unsorted,
    QueryName,
    Coordinate,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for SortOrder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "unsorted" => Ok(Self::Unsorted),
            "queryname" => Ok(Self::QueryName),
            "coordinate" => Ok(Self::Coordinate),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(SortOrder::default(), SortOrder::Unknown);
    }

    #[test]
    fn test_from_str() -> Result<(), ()> {
        assert_eq!("unknown".parse::<SortOrder>()?, SortOrder::Unknown);
        assert_eq!("unsorted".parse::<SortOrder>()?, SortOrder::Unsorted);
        assert_eq!("queryname".parse::<SortOrder>()?, SortOrder::QueryName);
        assert_eq!("coordinate".parse::<SortOrder>()?, SortOrder::Coordinate);

        assert!("".parse::<SortOrder>().is_err());
        assert!("noodles".parse::<SortOrder>().is_err());
        assert!("QueryName".parse::<SortOrder>().is_err());

        Ok(())
    }
}