use super::{FunctionDescriptor, Input, Output};
use crate::{
    data::{Data, DataType, ToData},
    expr::error::ExprError,
    utils::strings::DotDisplay,
};
use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};
use regex::bytes::Regex;
use rust_decimal::prelude::*;

fn string(i: Input) -> Output {
    i[0].display().data()
}

pub fn string_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: string,
        output: DataType::String,
    }
}

fn join(i: Input) -> Output {
    format!("{}{}", i[0], i[1]).data()
}

pub fn join_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any, DataType::Any],
        function: join,
        output: DataType::String,
    }
}

fn join_after(i: Input) -> Output {
    format!("{}{}", i[1], i[0]).data()
}

pub fn join_after_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any, DataType::Any],
        function: join_after,
        output: DataType::String,
    }
}

fn surround(i: Input) -> Output {
    format!("{}{}{}", i[1], i[0], i[2]).data()
}

pub fn surround_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any, DataType::Any, DataType::Any],
        function: surround,
        output: DataType::String,
    }
}

fn uppercase(i: Input) -> Output {
    i[0].string().to_uppercase().data()
}

pub fn uppercase_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: uppercase,
        output: DataType::String,
    }
}

fn lowercase(i: Input) -> Output {
    i[0].string().to_lowercase().data()
}

pub fn lowercase_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: lowercase,
        output: DataType::String,
    }
}

fn snake_case(i: Input) -> Output {
    i[0].string().to_snake_case().data()
}

pub fn snake_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: snake_case,
        output: DataType::String,
    }
}

fn kebab_case(i: Input) -> Output {
    i[0].string().to_kebab_case().data()
}

pub fn kebab_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: kebab_case,
        output: DataType::String,
    }
}

fn title_case(i: Input) -> Output {
    i[0].string().to_title_case().data()
}

pub fn title_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: title_case,
        output: DataType::String,
    }
}

fn upper_camel_case(i: Input) -> Output {
    i[0].string().to_upper_camel_case().data()
}

pub fn upper_camel_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: upper_camel_case,
        output: DataType::String,
    }
}

fn lower_camel_case(i: Input) -> Output {
    i[0].string().to_lower_camel_case().data()
}

pub fn lower_camel_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: lower_camel_case,
        output: DataType::String,
    }
}

fn shouty_kebab_case(i: Input) -> Output {
    i[0].string().to_shouty_kebab_case().data()
}

pub fn shouty_kebab_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: shouty_kebab_case,
        output: DataType::String,
    }
}

fn shouty_snake_case(i: Input) -> Output {
    i[0].string().to_shouty_snake_case().data()
}

pub fn shouty_snake_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: shouty_snake_case,
        output: DataType::String,
    }
}

fn train_case(i: Input) -> Output {
    i[0].string().to_train_case().data()
}

pub fn train_case_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: train_case,
        output: DataType::String,
    }
}

fn center(i: Input) -> Output {
    let s = i[0].string();
    let length = i[1].number().to_usize().unwrap();
    let char = i[2].string().bytes().next().unwrap() as char;
    let padding = (length - s.len()) / 2;
    let pad_str = char.to_string().repeat(padding);
    format!("{}{}{}", pad_str, s, pad_str).data()
}

pub fn center_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::Number, DataType::String],
        function: center,
        output: DataType::String,
    }
}

fn count(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();

    str.matches(search).count().data()
}

pub fn count_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: count,
        output: DataType::Number,
    }
}

fn ends_with(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();

    str.ends_with(search).data()
}

pub fn ends_with_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: ends_with,
        output: DataType::Bool,
    }
}

fn starts_with(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();

    str.starts_with(search).data()
}

pub fn starts_with_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: starts_with,
        output: DataType::Bool,
    }
}

fn find(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();

    str.find(search).map(|n| n as isize).unwrap_or(-1).data()
}

pub fn find_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: find,
        output: DataType::Number,
    }
}

fn is_alphanumeric(i: Input) -> Output {
    i[0].string().chars().all(char::is_alphanumeric).data()
}

pub fn is_alphanumeric_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_alphanumeric,
        output: DataType::Bool,
    }
}

fn is_alphabetic(i: Input) -> Output {
    i[0].string().chars().all(char::is_alphabetic).data()
}

pub fn is_alphabetic_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_alphabetic,
        output: DataType::Bool,
    }
}

fn is_ascii(i: Input) -> Output {
    i[0].string().is_ascii().data()
}

pub fn is_ascii_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_ascii,
        output: DataType::Bool,
    }
}

fn is_numeric(i: Input) -> Output {
    i[0].string().chars().all(char::is_numeric).data()
}

pub fn is_numeric_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_numeric,
        output: DataType::Bool,
    }
}

fn matches(i: Input) -> Output {
    let s = i[0].string();
    let regex = i[1].string();

    let regex = Regex::new(regex).map_err(|_| ExprError::InvalidRegex(regex.clone()))?;

    regex.is_match(s.as_bytes()).data()
}

pub fn matches_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: matches,
        output: DataType::Bool,
    }
}

fn is_lowercase(i: Input) -> Output {
    i[0].string().chars().all(char::is_lowercase).data()
}

pub fn is_lowercase_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_lowercase,
        output: DataType::Bool,
    }
}

fn is_uppercase(i: Input) -> Output {
    i[0].string().chars().all(char::is_uppercase).data()
}

pub fn is_uppercase_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_uppercase,
        output: DataType::Bool,
    }
}

fn is_whitespace(i: Input) -> Output {
    i[0].string().chars().all(char::is_whitespace).data()
}

pub fn is_whitespace_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: is_whitespace,
        output: DataType::Bool,
    }
}

fn trim(i: Input) -> Output {
    i[0].string().trim().data()
}

pub fn trim_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: trim,
        output: DataType::String,
    }
}

fn replace(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();
    let replace = i[2].string();

    str.replace(search, replace).data()
}

pub fn replace_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String, DataType::String],
        function: replace,
        output: DataType::String,
    }
}

fn split(i: Input) -> Output {
    let str = i[0].string();
    let search = i[1].string();

    str.split(search)
        .map(|s| s.data())
        .collect::<Result<Vec<Data>, _>>()
        .unwrap_or(vec![])
        .data()
}

pub fn split_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: split,
        output: DataType::Array,
    }
}
