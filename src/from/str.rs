use lazy_static::lazy_static;
use regex::Regex;
use std::{num::ParseFloatError, str::FromStr};
use thiserror::Error;

use crate::Complex;

lazy_static! {
    static ref PARSE_REGEX: Regex =
        Regex::new(r"^(?<real>-?[\d.]+)(?<imaginary>[+-]?[\d.]*)i$").unwrap();
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseComplexError {
    #[error("Couldn't match the given string as a complex number")]
    MatchError,
    #[error("Couldn't parse the real part ('{re}') of the given complex number")]
    ParseRealPartError { re: String, err: ParseFloatError },
    #[error("Couldn't parse the imaginary part ('{im}') of the given complex number")]
    ParseImaginaryPartError { im: String, err: ParseFloatError },
    #[error("The detection regex was invalid")]
    InvalidRegexError(#[from] ::regex::Error),
}

impl FromStr for Complex {
    type Err = ParseComplexError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        // get capture groups
        let captures = PARSE_REGEX
            .captures(source)
            // if none, convert to error and bubble up
            .ok_or(ParseComplexError::MatchError)?;

        let re: f64 =
            (&captures["real"])
                .parse()
                .map_err(|err| ParseComplexError::ParseRealPartError {
                    re: captures["real"].to_string(),
                    err,
                })?;

        let im: f64 = match &captures["imaginary"] {
            "-" => -1.,
            "+" => 1.,
            other => other
                .parse()
                .map_err(|err| ParseComplexError::ParseImaginaryPartError {
                    im: captures["imaginary"].to_string(),
                    err,
                })?,
        };

        Ok(Complex::from(re, im))
    }
}

#[cfg(test)]
mod test_parse_complex_number_from_string {
    use crate::{from::str::ParseComplexError, Complex};
    use std::str::FromStr;

    #[test]
    fn should_create_complex_numbers_from_strings() {
        let z1 = Complex::from_str("1+i").unwrap();
        assert_eq!(z1, Complex::from(1., 1.));
        let z2 = Complex::from_str("1-i").unwrap();
        assert_eq!(z2, Complex::from(1., -1.));
        let z3 = Complex::from_str("-1+1i").unwrap();
        assert_eq!(z3, Complex::from(-1., 1.));
        let z4 = Complex::from_str("-1-1i").unwrap();
        assert_eq!(z4, Complex::from(-1., -1.));
        let z5 = Complex::from_str("1.3+2.2i").unwrap();
        assert_eq!(z5, Complex::from(1.3, 2.2));
        let z6 = Complex::from_str("1.3-2.2i").unwrap();
        assert_eq!(z6, Complex::from(1.3, -2.2));
    }

    #[test]
    fn should_give_errors_when_parsing_malformed_strings() {
        let invalid_err = Complex::from_str("not a complex number").unwrap_err();
        assert_eq!(invalid_err, ParseComplexError::MatchError);
        let other_parts_err = Complex::from_str("its 3+2i").unwrap_err();
        assert_eq!(other_parts_err, ParseComplexError::MatchError);
    }

    #[test]
    fn should_use_string_parse() {
        let z = String::from("2+3i").parse::<Complex>().unwrap();
        assert_eq!(z, Complex::from(2., 3.));
    }
}
