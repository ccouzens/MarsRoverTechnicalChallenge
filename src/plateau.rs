type CoordinateUnit = i16;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsePlateauError {
    #[error("missing upper x")]
    MissingUpperX,
    #[error("missing upper y")]
    MissingUpperY,
    #[error("couldn't parse upper x")]
    UnparsableUpperX(ParseIntError),
    #[error("couldn't parse upper y")]
    UnparsableUpperY(ParseIntError),
}

#[derive(Error, Debug, PartialEq)]
pub enum OutOfPlataeuError {
    #[error("({x:}, {y:}) is not within the interval [(0, 0), ({upper_x:}, {upper_y:})]")]
    Outside {
        x: CoordinateUnit,
        y: CoordinateUnit,
        upper_x: CoordinateUnit,
        upper_y: CoordinateUnit,
    },
}

#[derive(Debug, PartialEq)]
pub struct Plateau {
    upper_x: CoordinateUnit,
    upper_y: CoordinateUnit,
}

impl FromStr for Plateau {
    type Err = ParsePlateauError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split_whitespace().map(str::parse);
        let upper_x = dimensions
            .next()
            .ok_or(ParsePlateauError::MissingUpperX)?
            .map_err(ParsePlateauError::UnparsableUpperX)?;
        let upper_y = dimensions
            .next()
            .ok_or(ParsePlateauError::MissingUpperY)?
            .map_err(ParsePlateauError::UnparsableUpperY)?;
        Ok(Plateau { upper_x, upper_y })
    }
}

impl Plateau {
    pub fn confirm_within(
        &self,
        x: CoordinateUnit,
        y: CoordinateUnit,
    ) -> Result<(), OutOfPlataeuError> {
        if x >= 0 && x <= self.upper_x && y >= 0 && y <= self.upper_y {
            Ok(())
        } else {
            Err(OutOfPlataeuError::Outside {
                x,
                y,
                upper_x: self.upper_x,
                upper_y: self.upper_y,
            })
        }
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
                upper_x: 4,
                upper_y: 2
            })
        );
    }
}
