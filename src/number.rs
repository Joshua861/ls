use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Rem, Sub, SubAssign},
};

use rand::Rng;

#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct Number {
    value: i64,
}

impl Number {
    const PRECISION: i64 = 100_000;

    // Idea for a better way to do this
    // const DECIMAL: i64 = 0b0000000000000000000000000000000011111111111111111111111111111111;
    // const INT: i64 = 0b1111111111111111111111111111111100000000000000000000000000000000;

    pub fn from_int(n: i64) -> Self {
        Self {
            value: n * Self::PRECISION,
        }
    }

    #[inline]
    pub fn new(n: i64) -> Self {
        Self { value: n }
    }

    pub fn sqrt(&self) -> Self {
        Self::from_f64(self.to_f64().sqrt())
    }

    pub fn to_f64(self) -> f64 {
        self.value as f64 / Self::PRECISION as f64
    }

    pub fn from_f64(n: f64) -> Self {
        Self::new((n * Self::PRECISION as f64) as i64)
    }

    pub fn from_int_with_decimal(int: i64, mut decimal: i64) -> Self {
        while decimal > Self::PRECISION {
            decimal = decimal.saturating_div(10);
        }

        while decimal < Self::PRECISION / 10 {
            decimal = decimal.saturating_mul(10);
        }

        Self::new(int * Self::PRECISION + decimal)
    }

    pub fn from_str(s: &str) -> Self {
        if let Some((l, r)) = s.split_once('.') {
            let v = l.parse().unwrap();
            let d = r.parse().unwrap();

            Self::from_int_with_decimal(v, d)
        } else {
            Self::from_int(s.parse().unwrap())
        }
    }

    pub fn rand() -> Self {
        Self::from_f64(rand::random())
    }

    pub fn decimal(&self) -> i64 {
        self.value % Self::PRECISION
    }

    pub fn int(&self) -> i64 {
        (self.value - self.decimal()) / Self::PRECISION
    }

    pub fn rand_between(bottom: Number, top: Number) -> Self {
        let top = top.int();
        let bottom = bottom.int();

        Number::from_int(rand::thread_rng().gen_range(top.min(bottom)..=top.max(bottom)))
    }

    pub fn abs(mut self) -> Self {
        self.value = self.value.abs();

        self
    }

    pub fn root(&self, expr: Self) -> Self {
        todo!()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Sub for Number {
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }

    type Output = Self;
}

impl Add for Number {
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }

    type Output = Self;
}

impl Mul for Number {
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_f64(self.to_f64() * rhs.to_f64())
    }

    type Output = Self;
}

impl Div for Number {
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_f64(self.to_f64() / rhs.to_f64())
    }

    type Output = Self;
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            value: self.value - rhs.value,
        }
    }
}

impl Neg for Number {
    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }

    type Output = Self;
}

impl Rem for Number {
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.value % rhs.value)
    }

    type Output = Self;
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.value.max(other.value))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.value.min(other.value))
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
