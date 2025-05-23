use gcd::Gcd;
use num::{FromPrimitive, ToPrimitive};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Fraction {
    numer: i32,
    denom: i32,
}

impl Fraction {
    pub fn new(numer: i32, denom: i32) -> Self {
        if denom == 0 {
            panic!("Denominator cannot be 0");
        }
        let mut fraction = Self { numer, denom };
        fraction.reduce();
        fraction
    }

    fn reduce(&mut self) {
        let numer = self.numer.unsigned_abs();
        let denom = self.denom.unsigned_abs();
        let gcd = numer.gcd(denom);
        self.numer /= gcd as i32;
        self.denom /= gcd as i32;
        if self.denom < 0 {
            self.numer = -self.numer;
            self.denom = -self.denom;
        }
    }

    pub fn multiply_to_int<T>(self, value: T) -> T
    where
        T: FromPrimitive + ToPrimitive + Mul<Output = T> + Div<Output = T>,
    {
        let numer = T::from_i32(self.numer).unwrap();
        let denom = T::from_i32(self.denom).unwrap();
        numer * value / denom
    }

    pub fn from_integer(numer: i32) -> Self {
        Self { numer, denom: 1 }
    }

    pub fn to_integer(self) -> i32 {
        self.numer / self.denom
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numer = self.numer * rhs.denom + self.denom * rhs.numer;
        let denom = self.denom * rhs.denom;
        Fraction::new(numer, denom)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let numer = self.numer * rhs.denom - self.denom * rhs.numer;
        let denom = self.denom * rhs.denom;
        Fraction::new(numer, denom)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let numer = self.numer * rhs.numer;
        let denom = self.denom * rhs.denom;
        Fraction::new(numer, denom)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let numer = self.numer * rhs.denom;
        let denom = self.denom * rhs.numer;
        Fraction::new(numer, denom)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        let numer = self.numer * rhs.denom + self.denom * rhs.numer;
        let denom = self.denom * rhs.denom;
        *self = Fraction::new(numer, denom);
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        let numer = self.numer * rhs.denom - self.denom * rhs.numer;
        let denom = self.denom * rhs.denom;
        *self = Fraction::new(numer, denom);
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        let numer = self.numer * rhs.numer;
        let denom = self.denom * rhs.denom;
        *self = Fraction::new(numer, denom);
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        let numer = self.numer * rhs.denom;
        let denom = self.denom * rhs.numer;
        *self = Fraction::new(numer, denom);
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.numer, self.denom)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numer * other.denom).cmp(&(other.numer * self.denom))
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numer, self.denom)
    }
}

impl FromStr for Fraction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid fraction".to_string());
        }
        let numer = parts[0]
            .parse::<i32>()
            .map_err(|_| format!("Invalid numerator: {}", parts[0]))?;
        let denom = parts[1]
            .parse::<i32>()
            .map_err(|_| format!("Invalid denominator: {}", parts[1]))?;
        Ok(Fraction::new(numer, denom))
    }
}

pub fn poison_damage(severity: u32) -> u32 {
    (severity + 4) / 5
}

pub fn lerp(
    current: i32,
    source_start: i32,
    source_end: i32,
    target_start: i32,
    target_end: i32,
) -> i32 {
    // Linear interpolation function
    target_start
        + (current - source_start) * (target_end - target_start) / (source_end - source_start)
}
