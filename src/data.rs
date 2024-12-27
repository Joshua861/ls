use std::fmt::{Display, Write};

use rust_decimal::Decimal;
use strum::EnumIs;

use crate::{expr::EResult, utils::strings::DotDisplay};

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Data {
    Number(Decimal),
    Bool(bool),
    Null,
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
    // Function,
    // Array,
}

// struct Array<T: Data> {
//     data: Vec<T>,
// }

impl Data {
    pub fn _type(&self) -> DataType {
        match self {
            Data::Number(_) => DataType::Number,
            Data::Bool(_) => DataType::Bool,
            Data::Null => DataType::Null,
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

pub fn format_types(types: Vec<DataType>) -> String {
    let mut t = String::new();

    write!(t, "(");
    let len = types.len();

    for (i, ty) in types.into_iter().enumerate() {
        if i < len - 1 {
            write!(t, "{}, ", ty);
        } else {
            write!(t, "{}", ty);
        }
    }

    write!(t, ")");

    t
}
