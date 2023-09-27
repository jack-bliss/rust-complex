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
mod test;
