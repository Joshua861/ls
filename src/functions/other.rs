use super::{FunctionDescriptor, FunctionType, Input, Output};
use crate::{
    data::{Data, DataType, ToData},
    utils::strings::DotDisplay,
};

fn println(i: Input) -> Output {
    println!("{}", i[0].display().replace("\\n", "\n"));

    Ok(Data::Null)
}

fn print(i: Input) -> Output {
    print!("{}", i[0].display().replace("\\n", "\n"));

    Ok(Data::Null)
}

pub fn print_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: FunctionType::BuiltIn(print),
        output: DataType::Null,
    }
}

pub fn println_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: FunctionType::BuiltIn(println),
        output: DataType::Null,
    }
}

fn type_of(i: Input) -> Output {
    i[0]._type().to_string().data()
}

pub fn type_of_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: FunctionType::BuiltIn(type_of),
        output: DataType::String,
    }
}

fn input(_i: Input) -> Output {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.data()
}

pub fn input_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![],
        function: FunctionType::BuiltIn(input),
        output: DataType::String,
    }
}

fn read_file(i: Input) -> Output {
    let input = std::fs::read_to_string(i[0].string()).unwrap();
    input.data()
}

pub fn read_file_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String],
        function: FunctionType::BuiltIn(read_file),
        output: DataType::String,
    }
}

pub fn write_file(i: Input) -> Output {
    std::fs::write(i[0].string(), i[1].string()).unwrap();

    Ok(Data::Null)
}

pub fn write_file_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::String, DataType::String],
        function: FunctionType::BuiltIn(write_file),
        output: DataType::Null,
    }
}
