use super::{FunctionDescriptor, Input, Output};
use crate::{
    data::{Data, DataType, ToData},
    utils::strings::DotDisplay,
};

fn print(i: Input) -> Output {
    println!("{}", i[0].display().replace("\\n", "\n"));

    Ok(Data::Null)
}

pub fn print_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: print,
        output: DataType::Null,
    }
}

fn type_of(i: Input) -> Output {
    i[0]._type().to_string().data()
}

pub fn type_of_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any],
        function: type_of,
        output: DataType::String,
    }
}
