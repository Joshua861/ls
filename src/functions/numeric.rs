use rand::Rng;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use super::{FunctionDescriptor, FunctionType, Input, Output};
use crate::{
    data::{DataType, ToData},
    expr::error::ExprError,
};

pub fn mod_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(mod_func),
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
        function: FunctionType::BuiltIn(add),
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
        function: FunctionType::BuiltIn(sub),
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
        function: FunctionType::BuiltIn(mul),
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
        function: FunctionType::BuiltIn(div),
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
        function: FunctionType::BuiltIn(neg),
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
        function: FunctionType::BuiltIn(sqrt),
        output: DataType::Number,
    }
}

pub fn abs(i: Input) -> Output {
    i[0].number().abs().data()
}

pub fn abs_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(abs),
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
        function: FunctionType::BuiltIn(abs_diff),
        output: DataType::Number,
    }
}

pub fn rand(_i: Input) -> Output {
    Decimal::from_f64(rand::random::<f64>()).unwrap().data()
}

pub fn rand_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![],
        function: FunctionType::BuiltIn(rand),
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
        function: FunctionType::BuiltIn(rand_between),
        output: DataType::Number,
    }
}

pub fn max(i: Input) -> Output {
    i[0].number().max(i[1].number()).data()
}

pub fn max_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(max),
        output: DataType::Number,
    }
}

pub fn min(i: Input) -> Output {
    i[0].number().min(i[1].number()).data()
}

pub fn min_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(min),
        output: DataType::Number,
    }
}

fn ceil(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().ceil())
        .unwrap()
        .data()
}

pub fn ceil_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(ceil),
        output: DataType::Number,
    }
}

fn floor(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().floor())
        .unwrap()
        .data()
}

pub fn floor_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(floor),
        output: DataType::Number,
    }
}

fn round(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().round())
        .unwrap()
        .data()
}

pub fn round_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(round),
        output: DataType::Number,
    }
}

fn pow(i: Input) -> Output {
    let a = i[0].number();
    let b = i[1].number();
    Decimal::from_f64(a.to_f64().unwrap().powf(b.to_f64().unwrap()))
        .unwrap()
        .data()
}

pub fn pow_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(pow),
        output: DataType::Number,
    }
}

fn sign(i: Input) -> Output {
    let a = i[0].number();
    if a.is_zero() {
        Decimal::ZERO
    } else if a.is_sign_positive() {
        Decimal::ONE
    } else {
        Decimal::NEGATIVE_ONE
    }
    .data()
}

pub fn sign_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(sign),
        output: DataType::Number,
    }
}

fn sin(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().sin()).unwrap().data()
}

pub fn sin_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(sin),
        output: DataType::Number,
    }
}

fn cos(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().cos()).unwrap().data()
}

pub fn cos_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(cos),
        output: DataType::Number,
    }
}

fn tan(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().tan()).unwrap().data()
}

pub fn tan_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(tan),
        output: DataType::Number,
    }
}

fn log(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().log10())
        .unwrap()
        .data()
}

pub fn log_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(log),
        output: DataType::Number,
    }
}

fn log2(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().log2())
        .unwrap()
        .data()
}

pub fn log2_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(log2),
        output: DataType::Number,
    }
}

fn log10(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().log10())
        .unwrap()
        .data()
}

pub fn log10_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(log10),
        output: DataType::Number,
    }
}

fn acos(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().acos())
        .unwrap()
        .data()
}

pub fn acos_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(acos),
        output: DataType::Number,
    }
}

fn acosh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().acosh())
        .unwrap()
        .data()
}

pub fn acosh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(acosh),
        output: DataType::Number,
    }
}

fn asin(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().asin())
        .unwrap()
        .data()
}

pub fn asin_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(asin),
        output: DataType::Number,
    }
}

fn asinh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().asinh())
        .unwrap()
        .data()
}

pub fn asinh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(asinh),
        output: DataType::Number,
    }
}

fn atan(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().atan())
        .unwrap()
        .data()
}

pub fn atan_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(atan),
        output: DataType::Number,
    }
}

fn atan2(i: Input) -> Output {
    let a = i[0].number();
    let b = i[1].number();
    Decimal::from_f64(a.to_f64().unwrap().atan2(b.to_f64().unwrap()))
        .unwrap()
        .data()
}

pub fn atan2_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(atan2),
        output: DataType::Number,
    }
}

fn atanh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().atanh())
        .unwrap()
        .data()
}

pub fn atanh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(atanh),
        output: DataType::Number,
    }
}

fn cbrt(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().cbrt())
        .unwrap()
        .data()
}

pub fn cbrt_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(cbrt),
        output: DataType::Number,
    }
}

fn cosh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().cosh())
        .unwrap()
        .data()
}

pub fn cosh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(cosh),
        output: DataType::Number,
    }
}

fn exp(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().exp()).unwrap().data()
}

pub fn exp_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(exp),
        output: DataType::Number,
    }
}

fn sinh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().sinh())
        .unwrap()
        .data()
}

pub fn sinh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(sinh),
        output: DataType::Number,
    }
}

fn tanh(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().tanh())
        .unwrap()
        .data()
}

pub fn tanh_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(tanh),
        output: DataType::Number,
    }
}

fn trunc(i: Input) -> Output {
    let a = i[0].number();
    Decimal::from_f64(a.to_f64().unwrap().trunc())
        .unwrap()
        .data()
}

pub fn trunc_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number],
        function: FunctionType::BuiltIn(trunc),
        output: DataType::Number,
    }
}

fn parse_to_number(i: Input) -> Output {
    Decimal::from_str(i[0].string())
        .map_err(|_| ExprError::BadNumber(i[0].string().clone()))?
        .data()
}

pub fn parse_to_number_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: FunctionType::BuiltIn(parse_to_number),
        output: DataType::Number,
    }
}
