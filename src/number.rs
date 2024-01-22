#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i32),
    // Quotient(i32, i32),
    Real(f64),
    // Gaussian(i32, i32),
    Complex(f64, f64),
}
