type CoordinateUnit = i16;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsePlateauError {
    #[error("missing width")]
    MissingWidth,
    #[error("missing height")]
    MissingHeight,
    #[error("couldn't parse width")]
    UnparsableWidth(ParseIntError),
    #[error("couldn't parse height")]
    UnparsableHeight(ParseIntError),
}

#[derive(Debug, PartialEq)]
pub struct Plateau {
    width: CoordinateUnit,
    height: CoordinateUnit,
}

impl FromStr for Plateau {
    type Err = ParsePlateauError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split_whitespace().map(str::parse);
        let width = dimensions
            .next()
            .ok_or(ParsePlateauError::MissingWidth)?
            .map_err(ParsePlateauError::UnparsableWidth)?;
        let height = dimensions
            .next()
            .ok_or(ParsePlateauError::MissingHeight)?
            .map_err(ParsePlateauError::UnparsableHeight)?;
        Ok(Plateau { width, height })
    }
}

#[cfg(test)]
mod tests {
    use crate::plateau::Plateau;

    #[test]
    fn can_be_parsed() {
        assert_eq!(
            "4 2".parse(),
            Ok(Plateau {
                width: 4,
                height: 2
            })
        );
    }
}
