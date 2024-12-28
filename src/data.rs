use std::fmt::{Display, Write};

use rust_decimal::Decimal;
use strum::EnumIs;

use crate::{expr::EResult, utils::strings::DotDisplay};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    Number(Decimal),
    Bool(bool),
    String(String),
    Null,
    Array(Vec<Data>),
    // Function(String),
    // Array(Array),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(d) => d.display(),
                Self::Bool(b) => b.display(),
                Self::Null => "null".into(),
                Self::String(s) => s.clone(),
                Self::Array(a) => format_vec(a),
            }
        )
    }
}

#[derive(Clone, Debug, Copy, PartialEq, EnumIs)]
pub enum DataType {
    Number,
    Bool,
    Null,
    Any,
    String,
    Array,
}

impl Data {
    pub fn _type(&self) -> DataType {
        match self {
            Data::Number(_) => DataType::Number,
            Data::Bool(_) => DataType::Bool,
            Data::Null => DataType::Null,
            Data::String(_) => DataType::String,
            Data::Array(_) => DataType::Array,
        }
    }

    /// USE WITH CAUTION: panics if input type is not bool!!!
    pub fn bool(&self) -> bool {
        match self {
            Data::Bool(b) => *b,
            _ => unreachable!(),
        }
    }

    /// USE WITH CAUTION: panics if input type is not number!!!
    pub fn number(&self) -> Decimal {
        match self {
            Data::Number(d) => *d,
            _ => unreachable!(),
        }
    }

    /// USE WITH CAUTION: panics if input type is not string!!!
    pub fn string(&self) -> &String {
        match self {
            Data::String(s) => s,
            _ => unreachable!(),
        }
    }

    /// USE WITH CAUTION: panics if input type is not array!!!
    pub fn array(&self) -> Vec<Data> {
        match self {
            Data::Array(a) => a.clone(),
            _ => unreachable!(),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait ToData {
    fn data(self) -> EResult<Data>;
}

impl ToData for Decimal {
    fn data(self) -> EResult<Data> {
        Ok(Data::Number(self))
    }
}

impl ToData for bool {
    fn data(self) -> EResult<Data> {
        Ok(Data::Bool(self))
    }
}

impl ToData for String {
    fn data(self) -> EResult<Data> {
        Ok(Data::String(self))
    }
}

impl ToData for Vec<Data> {
    fn data(self) -> EResult<Data> {
        Ok(Data::Array(self))
    }
}

impl ToData for usize {
    fn data(self) -> EResult<Data> {
        Ok(Data::Number(Decimal::from(self)))
    }
}

impl ToData for isize {
    fn data(self) -> EResult<Data> {
        Ok(Data::Number(Decimal::from(self)))
    }
}

impl ToData for &str {
    fn data(self) -> EResult<Data> {
        Ok(Data::String(self.to_string()))
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Number(a), Data::Number(b)) => a.cmp(b),
            (Data::String(a), Data::String(b)) => a.cmp(b),
            (Data::Array(a), Data::Array(b)) => a.cmp(b),
            _ => panic!("Cannot compare data types"),
        }
    }
}

pub fn format_types(types: Vec<DataType>) -> String {
    let mut t = String::new();

    write!(t, "(").unwrap();
    let len = types.len();

    for (i, ty) in types.into_iter().enumerate() {
        if i < len - 1 {
            write!(t, "{}, ", ty).unwrap();
        } else {
            write!(t, "{}", ty).unwrap();
        }
    }

    write!(t, ")").unwrap();

    t
}

pub fn format_vec<T>(v: &[T]) -> String
where
    T: Display,
{
    format!(
        "[{}]",
        &v.iter().map(|e| e.display()).collect::<Vec<_>>().join(", ")
    )
}
