use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::ParseError;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Integer(i32),
    Real(f64),
    // Gaussian(i32, i32),
    Complex(f64, f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{}", value),
            Self::Real(value) => write!(f, "{}", value),
            Self::Complex(real, im) => write!(f, "{}+{}i", real, im),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => lhs == rhs,
            (Self::Real(lhs), Self::Real(rhs)) => lhs == rhs,
            (Self::Complex(lhs_real, lhs_im), Self::Complex(rhs_real, rhs_im)) => {
                lhs_real == rhs_real && lhs_im == rhs_im
            }
            (Self::Integer(lhs), Self::Real(rhs)) => lhs as f64 == rhs,
            (Self::Real(lhs), Self::Integer(rhs)) => lhs == rhs as f64,
            (Self::Integer(lhs), Self::Complex(rhs_real, rhs_im)) => {
                lhs as f64 == rhs_real && rhs_im == 0.0
            }
            (Self::Complex(lhs_real, lhs_im), Self::Integer(rhs)) => {
                lhs_real == rhs as f64 && lhs_im == 0.0
            }
            (Self::Real(lhs), Self::Complex(rhs_real, rhs_im)) => lhs == rhs_real && rhs_im == 0.0,
            (Self::Complex(lhs_real, lhs_im), Self::Real(rhs)) => lhs_real == rhs && lhs_im == 0.0,
        }
    }
}

impl Eq for Number {}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + rhs),
            (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs + rhs),
            (Self::Complex(lhs_real, lhs_im), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs_real + rhs_real, lhs_im + rhs_im)
            }
            (Self::Integer(lhs), Self::Real(rhs)) => Self::Real(lhs as f64 + rhs),
            (Self::Real(lhs), Self::Integer(rhs)) => Self::Real(lhs + rhs as f64),
            (Self::Integer(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs as f64 + rhs_real, rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Integer(rhs)) => {
                Self::Complex(lhs_real + rhs as f64, lhs_im)
            }
            (Self::Real(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs + rhs_real, rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Real(rhs)) => {
                Self::Complex(lhs_real + rhs, lhs_im)
            }
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs - rhs),
            (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs - rhs),
            (Self::Complex(lhs_real, lhs_im), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs_real - rhs_real, lhs_im - rhs_im)
            }
            (Self::Integer(lhs), Self::Real(rhs)) => Self::Real(lhs as f64 - rhs),
            (Self::Real(lhs), Self::Integer(rhs)) => Self::Real(lhs - rhs as f64),
            (Self::Integer(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs as f64 - rhs_real, rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Integer(rhs)) => {
                Self::Complex(lhs_real - rhs as f64, lhs_im)
            }
            (Self::Real(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs - rhs_real, rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Real(rhs)) => {
                Self::Complex(lhs_real - rhs, lhs_im)
            }
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs * rhs),
            (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs * rhs),
            (Self::Complex(lhs_real, lhs_im), Self::Complex(rhs_real, rhs_im)) => Self::Complex(
                lhs_real * rhs_real - lhs_im * rhs_im,
                lhs_real * rhs_im + lhs_im * rhs_real,
            ),
            (Self::Integer(lhs), Self::Real(rhs)) => Self::Real(lhs as f64 * rhs),
            (Self::Real(lhs), Self::Integer(rhs)) => Self::Real(lhs * rhs as f64),
            (Self::Integer(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs as f64 * rhs_real, lhs as f64 * rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Integer(rhs)) => {
                Self::Complex(lhs_real * rhs as f64, lhs_im * rhs as f64)
            }
            (Self::Real(lhs), Self::Complex(rhs_real, rhs_im)) => {
                Self::Complex(lhs * rhs_real, lhs * rhs_im)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Real(rhs)) => {
                Self::Complex(lhs_real * rhs, lhs_im * rhs)
            }
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs / rhs),
            (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs / rhs),
            (Self::Complex(lhs_real, lhs_im), Self::Complex(rhs_real, rhs_im)) => {
                let denominator = rhs_real * rhs_real + rhs_im * rhs_im;
                Self::Complex(
                    (lhs_real * rhs_real + lhs_im * rhs_im) / denominator,
                    (lhs_im * rhs_real - lhs_real * rhs_im) / denominator,
                )
            }
            (Self::Integer(lhs), Self::Real(rhs)) => Self::Real(lhs as f64 / rhs),
            (Self::Real(lhs), Self::Integer(rhs)) => Self::Real(lhs / rhs as f64),
            (Self::Integer(lhs), Self::Complex(rhs_real, rhs_im)) => {
                let denominator = rhs_real * rhs_real + rhs_im * rhs_im;
                Self::Complex(
                    lhs as f64 * rhs_real / denominator,
                    -lhs as f64 * rhs_im / denominator,
                )
            }
            (Self::Complex(lhs_real, lhs_im), Self::Integer(rhs)) => {
                Self::Complex(lhs_real / rhs as f64, lhs_im / rhs as f64)
            }
            (Self::Real(lhs), Self::Complex(rhs_real, rhs_im)) => {
                let denominator = rhs_real * rhs_real + rhs_im * rhs_im;
                Self::Complex(lhs * rhs_real / denominator, -lhs * rhs_im / denominator)
            }
            (Self::Complex(lhs_real, lhs_im), Self::Real(rhs)) => {
                Self::Complex(lhs_real / rhs, lhs_im / rhs)
            }
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(value) => Self::Integer(-value),
            Self::Real(value) => Self::Real(-value),
            Self::Complex(real, im) => Self::Complex(-real, -im),
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl From<(f64, f64)> for Number {
    fn from(value: (f64, f64)) -> Self {
        Self::Complex(value.0, value.1)
    }
}

impl TryFrom<&str> for Number {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        const COMPLEX: Lazy<[Regex; 6]> = Lazy::new(|| {
            [
                Regex::new(r#"^(?P<real>[+-]?[0-9]+(\.[0-9]+)?)(?P<im>[+-][0-9]+(\.[0-9]+)?)i$"#)
                    .unwrap(),
                Regex::new(r#"^(?P<im>[+-]?[0-9]+(\.[0-9]+)?)i(?P<real>[+-][0-9]+(\.[0-9]+)?)$"#)
                    .unwrap(),
                Regex::new(r#"^(?P<real>[+-]?[0-9]+(\.[0-9]+)?)@(?P<im>[+-]?[0-9]+(\.[0-9]+)?)$"#)
                    .unwrap(),
                Regex::new(r#"^(?<real>[+-]?[0-9]+(\.[0-9]+)?)[+-]i$"#).unwrap(),
                Regex::new(r#"^(?<im>[+-]?[0-9]+(\.[0-9]+)?)i$"#).unwrap(),
                Regex::new(r#"^[+-]i$"#).unwrap(),
            ]
        });
        if let Ok(value) = value.parse::<i32>() {
            Ok(Self::Integer(value))
        } else if let Ok(value) = value.parse::<f64>() {
            Ok(Self::Real(value))
        } else if let Some(caps) = COMPLEX
            .iter()
            .map(|pattern| pattern.captures(value))
            .skip_while(|res| res.is_none())
            .next()
            .flatten()
        {
            let real = match caps.name("real") {
                Some(real) => real
                    .as_str()
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(value.to_string()))?,
                None => 0.0,
            };
            let im = match caps.name("im") {
                Some(im) => im
                    .as_str()
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(value.to_string()))?,
                None => 1.0,
            };
            Ok(Self::Complex(real, im))
        } else {
            Err(ParseError::InvalidNumber(value.to_string()))
        }
    }
}
