use std::collections::HashMap;

mod numeric;
pub use numeric::*;

mod boolean;
pub use boolean::*;

mod other;
pub use other::*;

mod string;
pub use string::*;

mod array;
pub use array::*;

use crate::{
    data::{Data, DataType},
    expr::{EResult, Expr},
};

pub type Input = Vec<Data>;
pub type Output = EResult<Data>;
pub type FunctionMap = HashMap<String, FunctionDescriptor>;

#[derive(Debug, Clone)]
pub struct FunctionDescriptor {
    pub inputs: Vec<DataType>,
    pub function: FunctionType,
    pub output: DataType,
}

#[derive(Clone, Debug)]
pub enum FunctionType {
    BuiltIn(fn(Input) -> Output),
    Custom(Vec<Expr>, Vec<String>),
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
        ("round", round_descriptor()),
        ("ceil", ceil_descriptor()),
        ("floor", floor_descriptor()),
        ("pow", pow_descriptor()),
        ("sign", sign_descriptor()),
        ("sin", sin_descriptor()),
        ("cos", cos_descriptor()),
        ("tan", tan_descriptor()),
        ("log", log_descriptor()),
        ("log10", log10_descriptor()),
        ("log2", log2_descriptor()),
        ("trunc", trunc_descriptor()),
        ("tanh", tanh_descriptor()),
        ("exp", exp_descriptor()),
        ("sinh", sinh_descriptor()),
        ("cosh", cosh_descriptor()),
        ("tanh", tanh_descriptor()),
        ("cbrt", cbrt_descriptor()),
        ("atanh", atanh_descriptor()),
        ("atan", atan_descriptor()),
        ("atan2", atan2_descriptor()),
        ("asin", asin_descriptor()),
        ("asinh", asinh_descriptor()),
        ("acos", acos_descriptor()),
        ("acosh", acosh_descriptor()),
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
        // string
        ("join", join_descriptor()),
        ("join_after", join_after_descriptor()),
        ("surround", surround_descriptor()),
        ("string", string_descriptor()),
        ("center", center_descriptor()),
        ("count", count_descriptor()),
        ("ends_with", ends_with_descriptor()),
        ("starts_with", starts_with_descriptor()),
        ("find", find_descriptor()),
        ("is_numeric", is_numeric_descriptor()),
        ("is_alphanumeric", is_alphanumeric_descriptor()),
        ("is_alphabetic", is_alphabetic_descriptor()),
        ("is_ascii", is_ascii_descriptor()),
        ("matches", matches_descriptor()),
        ("is_lowercase", is_lowercase_descriptor()),
        ("is_uppercase", is_uppercase_descriptor()),
        ("is_whitespace", is_whitespace_descriptor()),
        ("trim", trim_descriptor()),
        ("replace", replace_descriptor()),
        ("split", split_descriptor()),
        ("uppercase", uppercase_descriptor()),
        ("lowercase", lowercase_descriptor()),
        ("upper_camel_case", upper_camel_case_descriptor()),
        ("lower_camel_case", lower_camel_case_descriptor()),
        ("snake_case", snake_case_descriptor()),
        ("kebab_case", kebab_case_descriptor()),
        ("shouty_kebab_case", shouty_kebab_case_descriptor()),
        ("shouty_snake_case", shouty_snake_case_descriptor()),
        ("title_case", title_case_descriptor()),
        ("train_case", train_case_descriptor()),
        // array
        ("join_array", join_array_descriptor()),
        ("sort", sort_descriptor()),
        ("length", length_descriptor()),
        ("index", index_descriptor()),
        ("append", append_descriptor()),
        ("flatten", flatten_descriptor()),
        ("extend", extend_descriptor()),
        ("reverse", reverse_descriptor()),
        ("without", without_descriptor()),
        ("with_insert", with_insert_descriptor()),
        ("range", range_descriptor()),
        ("max_array", max_array_descriptor()),
        ("min_array", min_array_descriptor()),
        // other
        ("type", type_of_descriptor()),
        ("print", print_descriptor()),
        ("println", println_descriptor()),
        ("input", input_descriptor()),
        ("read_file", read_file_descriptor()),
        ("write_file", write_file_descriptor()),
    ] {
        map.insert(name.to_string(), descriptor);
    }

    map
}
