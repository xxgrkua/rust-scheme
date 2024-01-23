use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i32),
    // Quotient(i32, i32),
    Real(f64),
    // Gaussian(i32, i32),
    Complex(f64, f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(value) => write!(f, "{}", value),
            Number::Real(value) => write!(f, "{}", value),
            Number::Complex(real, im) => write!(f, "{}+{}i", real, im),
        }
    }
}
