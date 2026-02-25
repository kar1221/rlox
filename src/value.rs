use std::ops;

#[derive(Clone, Debug, Copy)]
pub struct Value(f64);

impl Value {
    pub fn number(n: f64) -> Self { Self(n) }
    pub fn as_f64(self) -> f64 { self.0 }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}
