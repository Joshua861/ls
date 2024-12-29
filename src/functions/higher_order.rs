use super::{FunctionDescriptor, FunctionType, Input, Output};
use crate::{
    data::{format_types, Data, DataType, ToData},
    execute_block,
    expr::{error::ExprError, EResult, ExecutionState},
};

fn run(inputs: Vec<Data>, func: FunctionDescriptor, state: &ExecutionState) -> EResult<Data> {
    let matching_types = inputs
        .iter()
        .map(|i| i._type())
        .zip(func.inputs.iter())
        .all(|(input, expected)| input == *expected || expected.is_any());

    if matching_types {
        Ok(match func.function {
            FunctionType::BuiltIn(f) => f(inputs)?,
            FunctionType::Custom(block, input_names) => {
                let mut state = state.clone();

                for (i, name) in input_names.iter().enumerate() {
                    state.variables.insert(name.clone(), inputs[i].clone());
                }

                execute_block(&block, &state).0
            }
        })
    } else {
        let input_types = inputs.iter().map(|i| i._type()).collect::<Vec<_>>();
        Err(ExprError::InvalidFunctionArguements {
            expected: format_types(func.inputs),
            found: format_types(input_types),
            // name:  name.clone(),
        })
    }
}

fn map(i: Input) -> Output {
    let array = i[0].array().clone();
    let func = i[1].function().clone();

    array
        .iter()
        .map(|i| run(vec![i.clone()], func.clone(), &ExecutionState::new()))
        .collect::<EResult<Vec<_>>>()
        .map(|i| i.data())?
}

pub fn map_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Function],
        function: FunctionType::BuiltIn(map),
        output: DataType::Array,
    }
}

fn for_each(i: Input) -> Output {
    let array = i[0].array().clone();
    let func = i[1].function().clone();

    array.iter().for_each(|i| {
        run(vec![i.clone()], func.clone(), &ExecutionState::new()).unwrap();
    });

    Ok(Data::Null)
}

pub fn for_each_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Function],
        function: FunctionType::BuiltIn(for_each),
        output: DataType::Null,
    }
}

fn filter(i: Input) -> Output {
    let array = i[0].array().clone();
    let func = i[1].function().clone();

    array
        .iter()
        .filter(|&i| {
            run(vec![i.clone()], func.clone(), &ExecutionState::new())
                .unwrap()
                .is_true()
        })
        .collect::<Vec<_>>()
        .data()
}

pub fn filter_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Function],
        function: FunctionType::BuiltIn(filter),
        output: DataType::Array,
    }
}

fn fold(i: Input) -> Output {
    let array = i[0].array().clone();
    let initial = i[1].clone();
    let func = i[2].function().clone();

    array
        .iter()
        .fold(initial, |acc, i| {
            run(
                vec![acc.clone(), i.clone()],
                func.clone(),
                &ExecutionState::new(),
            )
            .unwrap()
        })
        .data()
}

pub fn fold_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Array, DataType::Any, DataType::Function],
        function: FunctionType::BuiltIn(fold),
        output: DataType::Null,
    }
}
