use super::{FunctionDescriptor, FunctionType, Input, Output};
use crate::{
    data::{Data, DataType, ToData},
    expr::error::ExprError,
    utils::strings::DotDisplay,
};
use rust_decimal::prelude::ToPrimitive;

fn join_array(i: Input) -> Output {
    let a = i[0].array();
    let sep = i[1].string();

    a.iter()
        .map(|i| i.display())
        .collect::<Vec<String>>()
        .join(sep)
        .data()
}

pub fn join_array_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::String],
        function: FunctionType::BuiltIn(join_array),
        output: DataType::String,
    }
}

fn sort(i: Input) -> Output {
    let mut a = i[0].array();
    a.sort();

    a.data()
}

pub fn sort_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(sort),
        output: DataType::Array,
    }
}

fn length(i: Input) -> Output {
    i[0].array().len().data()
}

pub fn length_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(length),
        output: DataType::Number,
    }
}

fn index(i: Input) -> Output {
    let a = i[0].array();
    let i = i[1].number().to_usize().unwrap();
    Ok(a[i].clone())
}

pub fn index_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Number],
        function: FunctionType::BuiltIn(index),
        output: DataType::Any,
    }
}

fn append(i: Input) -> Output {
    let mut a = i[0].array();
    let b = i[1].clone();

    a.push(b);

    a.data()
}

pub fn append_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Any],
        function: FunctionType::BuiltIn(append),
        output: DataType::Array,
    }
}

fn flatten(i: Input) -> Output {
    let mut a = Vec::new();
    for i in i[0].array() {
        match i {
            Data::Array(b) => a.extend(b),
            i => a.push(i.clone()),
        }
    }

    a.data()
}

pub fn flatten_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(flatten),
        output: DataType::Array,
    }
}

fn reverse(i: Input) -> Output {
    let mut a = i[0].array();
    a.reverse();

    a.data()
}

pub fn reverse_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(reverse),
        output: DataType::Array,
    }
}

fn extend(i: Input) -> Output {
    let mut a = i[0].array();
    a.extend(i[1].array());

    a.data()
}

pub fn extend_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Array],
        function: FunctionType::BuiltIn(extend),
        output: DataType::Array,
    }
}

fn without(i: Input) -> Output {
    let mut a = i[0].array();
    let index = i[1].number();

    a.remove(index.to_usize().unwrap());

    a.data()
}

pub fn without_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Number],
        function: FunctionType::BuiltIn(without),
        output: DataType::Array,
    }
}

fn with_insert(i: Input) -> Output {
    let mut a = i[0].array();
    let index = i[1].number();
    let item = i[2].clone();

    a.insert(index.to_usize().unwrap(), item);

    a.data()
}

pub fn with_insert_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Number, DataType::Any],
        function: FunctionType::BuiltIn(with_insert),
        output: DataType::Array,
    }
}

fn range(i: Input) -> Output {
    ((i[0].number().to_usize().unwrap())..(i[1].number().to_usize().unwrap()))
        .collect::<Vec<_>>()
        .data()
}

pub fn range_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(range),
        output: DataType::Array,
    }
}

fn max_array(i: Input) -> Output {
    let mut a = i[0].array();
    a.sort();

    a.pop().ok_or(ExprError::ArrayIsEmpty)
}

pub fn max_array_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(max_array),
        output: DataType::Any,
    }
}

fn min_array(i: Input) -> Output {
    let mut a = i[0].array();
    a.sort();
    a.reverse();
    a.pop().ok_or(ExprError::ArrayIsEmpty)
}

pub fn min_array_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(min_array),
        output: DataType::Any,
    }
}

fn first(i: Input) -> Output {
    i[0].array().first().cloned().ok_or(ExprError::ArrayIsEmpty)
}

pub fn first_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(first),
        output: DataType::Any,
    }
}

fn last(i: Input) -> Output {
    i[0].array().last().cloned().ok_or(ExprError::ArrayIsEmpty)
}

pub fn last_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array],
        function: FunctionType::BuiltIn(last),
        output: DataType::Any,
    }
}
