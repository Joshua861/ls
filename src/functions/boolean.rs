use super::{FunctionDescriptor, FunctionType, Input, Output};
use crate::data::{DataType, ToData};

pub fn and(i: Input) -> Output {
    (i[0].bool() && i[1].bool()).data()
}

pub fn and_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Bool, DataType::Bool],
        function: FunctionType::BuiltIn(and),
        output: DataType::Bool,
    }
}

pub fn or(i: Input) -> Output {
    (i[0].bool() || i[1].bool()).data()
}

pub fn or_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Bool, DataType::Bool],
        function: FunctionType::BuiltIn(or),
        output: DataType::Bool,
    }
}

pub fn eq(i: Input) -> Output {
    (i[0] == i[1]).data()
}

pub fn eq_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any, DataType::Any],
        function: FunctionType::BuiltIn(eq),
        output: DataType::Bool,
    }
}

pub fn ne(i: Input) -> Output {
    (i[0] != i[1]).data()
}

pub fn ne_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Any, DataType::Any],
        function: FunctionType::BuiltIn(ne),
        output: DataType::Bool,
    }
}

pub fn not(i: Input) -> Output {
    (!i[0].bool()).data()
}

pub fn not_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Bool],
        function: FunctionType::BuiltIn(not),
        output: DataType::Bool,
    }
}

pub fn xor(i: Input) -> Output {
    (i[0].bool() ^ i[1].bool()).data()
}

pub fn xor_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Bool, DataType::Bool],
        function: FunctionType::BuiltIn(xor),
        output: DataType::Bool,
    }
}

pub fn gt(i: Input) -> Output {
    (i[0].number().gt(&i[1].number())).data()
}

pub fn gt_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(gt),
        output: DataType::Bool,
    }
}

pub fn lt(i: Input) -> Output {
    (i[0].number().lt(&i[1].number())).data()
}

pub fn lt_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(lt),
        output: DataType::Bool,
    }
}

pub fn ge(i: Input) -> Output {
    (i[0].number().ge(&i[1].number())).data()
}

pub fn ge_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(ge),
        output: DataType::Bool,
    }
}

pub fn le(i: Input) -> Output {
    (i[0].number().le(&i[1].number())).data()
}

pub fn le_descriptor() -> FunctionDescriptor {
    FunctionDescriptor {
        inputs: vec![DataType::Number, DataType::Number],
        function: FunctionType::BuiltIn(le),
        output: DataType::Bool,
    }
}
