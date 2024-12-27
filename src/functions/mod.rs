use std::collections::HashMap;

mod numeric;
pub use numeric::*;

mod boolean;
pub use boolean::*;

use crate::{
    data::{Data, DataType},
    expr::EResult,
};

pub type Input = Vec<Data>;
pub type Output = EResult<Data>;
pub type FunctionMap = HashMap<String, FunctionDescriptor>;

#[derive(Debug, Clone)]
pub struct FunctionDescriptor {
    pub inputs: Vec<DataType>,
    pub function: fn(Input) -> Output,
    pub output: DataType,
}

pub fn builtints() -> FunctionMap {
    let mut map = HashMap::new();

    for (name, descriptor) in [
        // numeric
        ("sqrt", sqrt_descriptor()),
        ("abs", abs_descriptor()),
        ("abs_diff", abs_diff_descriptor()),
        ("rand", rand_descriptor()),
        ("rand_between", rand_between_descriptor()),
        ("max", max_descriptor()),
        ("min", min_descriptor()),
        ("add", add_descriptor()),
        ("sub", sub_descriptor()),
        ("div", div_descriptor()),
        ("mul", mul_descriptor()),
        ("neg", neg_descriptor()),
        ("mod", mod_descriptor()),
        // boolean
        ("or", or_descriptor()),
        ("and", and_descriptor()),
        ("not", not_descriptor()),
        ("xor", xor_descriptor()),
        ("ne", ne_descriptor()),
        ("eq", eq_descriptor()),
        ("ge", ge_descriptor()),
        ("le", le_descriptor()),
        ("gt", gt_descriptor()),
        ("lt", lt_descriptor()),
    ] {
        map.insert(name.to_string(), descriptor);
    }

    map
}
