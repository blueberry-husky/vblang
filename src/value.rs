use std::{
    borrow::Cow,
    ops::{self, Add, Div, Mul, Rem, Sub},
};

use core::fmt;
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Integer(isize),
    Float(f64),
    String(String),
    Uninitialized,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Integer(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            String(str) => write!(f, "{}", str),
            Uninitialized => write!(f, "UNINITIALIZED"),
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        use Value::*;
        match (self, other) {
            (Uninitialized, _) => panic!("Attempt to add involving an UNINITIALIZED value"),
            (_, Uninitialized) => panic!("Attempt to add involving an UNINITIALIZED value"),
            (String(str1), any) => String(format!("{str1}{any}")),
            (any1, String(str)) => String(format!("{any1}{str}")),
            (Float(flt1), Float(flt2)) => Float(flt1 + flt2),
            (Float(flt1), Integer(int2)) => Float(flt1 + int2 as f64),
            (Integer(int1), Float(flt2)) => Float(int1 as f64 + flt2),
            (Integer(int1), Integer(int2)) => Integer(int1 + int2),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        use Value::*;
        match (self, other) {
            (Uninitialized, _) => panic!("Attempt to subtract involving an UNINITIALIZED value"),
            (_, Uninitialized) => panic!("Attempt to subtract involving an UNINITIALIZED value"),
            (String(_), _) => panic!("Attempt to subtract involving a string value"),
            (_, String(_)) => panic!("Attempt to subtract involving a string value"),
            (Float(flt1), Float(flt2)) => Float(flt1 - flt2),
            (Float(flt1), Integer(int2)) => Float(flt1 - int2 as f64),
            (Integer(int1), Float(flt2)) => Float(int1 as f64 - flt2),
            (Integer(int1), Integer(int2)) => Integer(int1 - int2),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        use Value::*;
        match (self, other) {
            (Uninitialized, _) => panic!("Attempt to multiply involving an UNINITIALIZED value"),
            (_, Uninitialized) => panic!("Attempt to multiply involving an UNINITIALIZED value"),
            (String(_), _) => panic!("Attempt to multiply involving a string value"),
            (_, String(_)) => panic!("Attempt to multiply involving a string value"),
            (Float(flt1), Float(flt2)) => Float(flt1 * flt2),
            (Float(flt1), Integer(int2)) => Float(flt1 * int2 as f64),
            (Integer(int1), Float(flt2)) => Float(int1 as f64 * flt2),
            (Integer(int1), Integer(int2)) => Integer(int1 * int2),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        use Value::*;
        match (self, other) {
            (Uninitialized, _) => panic!("Attempt to divide involving an UNINITIALIZED value"),
            (_, Uninitialized) => panic!("Attempt to divide involving an UNINITIALIZED value"),
            (String(_), _) => panic!("Attempt to divide involving a string value"),
            (_, String(_)) => panic!("Attempt to divide involving a string value"),
            (Float(flt1), Float(flt2)) => Float(flt1 / flt2),
            (Float(flt1), Integer(int2)) => Float(flt1 / int2 as f64),
            (Integer(int1), Float(flt2)) => Float(int1 as f64 / flt2),
            (Integer(int1), Integer(int2)) => Integer(int1 / int2),
        }
    }
}

impl Rem for Value {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        use Value::*;
        match (self, other) {
            (Uninitialized, _) => panic!("Attempt to divide involving an UNINITIALIZED value"),
            (_, Uninitialized) => panic!("Attempt to divide involving an UNINITIALIZED value"),
            (String(_), _) => panic!("Attempt to divide involving a string value"),
            (_, String(_)) => panic!("Attempt to divide involving a string value"),
            (Float(flt1), Float(flt2)) => Float(flt1 % flt2),
            (Float(flt1), Integer(int2)) => Float(flt1 % int2 as f64),
            (Integer(int1), Float(flt2)) => Float(int1 as f64 % flt2),
            (Integer(int1), Integer(int2)) => Integer(int1 % int2),
        }
    }
}

pub fn interpret_string<'a>(string: Cow<'a, str>) -> Value {
    use Value::*;
    if let Ok(value) = string.trim().parse::<isize>() {
        Integer(value)
    } else {
        if let Ok(value) = string.trim().parse() {
            Float(value)
        } else {
            String(string.to_string())
        }
    }
}
