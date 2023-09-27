use regex::Regex;
use std::{num::ParseFloatError, str::FromStr};

use crate::Complex;

#[derive(Debug, Clone)]
pub struct MatchError;

#[derive(Debug, Clone)]
pub enum ParseComplexError {
    MatchError,
    ParseRealPartError(String, ParseFloatError),
    ParseImaginaryPartError(String, ParseFloatError),
}

impl FromStr for Complex {
    type Err = ParseComplexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let complex_number_reg_ex =
            Regex::new(r"(?<real>-?[\d.]+)(?<imaginary>[+-]?[\d.]*)i").unwrap();
        let captures = match complex_number_reg_ex.captures(s) {
            Some(captures) => captures,
            None => return Err(ParseComplexError::MatchError),
        };
        let re: f64 = match captures["real"].parse() {
            Ok(value) => value,
            Err(err) => {
                return Err(ParseComplexError::ParseRealPartError(
                    captures["real"].to_string(),
                    err,
                ))
            }
        };
        let im: f64 = match &captures["imaginary"] {
            "-" => -1.,
            "+" => 1.,
            other => match other.parse() {
                Ok(value) => value,
                Err(err) => {
                    return Err(ParseComplexError::ParseImaginaryPartError(
                        captures["imaginary"].to_string(),
                        err,
                    ))
                }
            },
        };

        return Ok(Complex::from(re, im));
    }
}

#[cfg(test)]
mod test;
