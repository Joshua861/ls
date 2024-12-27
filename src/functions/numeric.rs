use rand::Rng;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use super::{FunctionDescriptor, Input, Output};
use crate::{
    data::{DataType, ToData},
    expr::error::ExprError,
};

pub fn mod_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: mod_func,
        output: DataType::Number,
    }
}

pub fn mod_func(i: Input) -> Output {
    let rhs = i[0].number();
    let lhs = i[1].number();

    (lhs % rhs).data()
}

pub fn add_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: add,
        output: DataType::Number,
    }
}

pub fn add(i: Input) -> Output {
    let rhs = i[0].number();
    let lhs = i[1].number();

    (lhs + rhs).data()
}

pub fn sub_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: sub,
        output: DataType::Number,
    }
}

pub fn sub(i: Input) -> Output {
    let rhs = i[0].number();
    let lhs = i[1].number();

    (lhs - rhs).data()
}

pub fn mul_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: mul,
        output: DataType::Number,
    }
}

pub fn mul(i: Input) -> Output {
    let rhs = i[0].number();
    let lhs = i[1].number();

    (lhs * rhs).data()
}

pub fn div_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: div,
        output: DataType::Number,
    }
}

pub fn div(i: Input) -> Output {
    let rhs = i[0].number();
    let lhs = i[1].number();

    if rhs == dec!(0) {
        Err(ExprError::DivideBy0)
    } else {
        (lhs / rhs).data()
    }
}

pub fn neg_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: neg,
        output: DataType::Number,
    }
}

pub fn neg(i: Input) -> Output {
    (-i[0].number()).data()
}

pub fn sqrt(i: Input) -> Output {
    Decimal::from_f64(i[0].number().to_f64().unwrap().sqrt())
        .unwrap()
        .data()
}

pub fn sqrt_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: sqrt,
        output: DataType::Number,
    }
}

pub fn abs(i: Input) -> Output {
    i[0].number().abs().data()
}

pub fn abs_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: abs,
        output: DataType::Number,
    }
}

pub fn abs_diff(i: Input) -> Output {
    let a = i[0].number();
    let b = i[1].number();

    (a.max(b) - a.min(b)).data()
}

pub fn abs_diff_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: abs_diff,
        output: DataType::Number,
    }
}

pub fn rand(_i: Input) -> Output {
    Decimal::from_f64(rand::random::<f64>()).unwrap().data()
}

pub fn rand_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![],
        function: rand,
        output: DataType::Number,
    }
}

pub fn rand_between(i: Input) -> Output {
    let top = i[0].number();
    let bottom = i[1].number();

    let top = top.to_i64().unwrap();
    let bottom = bottom.to_i64().unwrap();

    (Decimal::from_i64(rand::thread_rng().gen_range(top.min(bottom)..=top.max(bottom))).unwrap())
        .data()
}

pub fn rand_between_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: rand_between,
        output: DataType::Number,
    }
}

pub fn max(i: Input) -> Output {
    i[0].number().max(i[1].number()).data()
}

pub fn max_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: max,
        output: DataType::Number,
    }
}

pub fn min(i: Input) -> Output {
    i[0].number().min(i[1].number()).data()
}

pub fn min_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: min,
        output: DataType::Number,
    }
}
