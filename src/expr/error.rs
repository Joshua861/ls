use thiserror::Error;

use crate::data::DataType;

#[derive(Debug, Error)]
pub enum ExprError {
    #[error("Attempted to divide by 0.")]
    DivideBy0,

    #[error("Invalid arguements passed to function. Expected: {expected}, found: {found}.")]
    InvalidFunctionArguements {
        expected: String,
        found: String,
        // name: String,
    },

    #[error("Function `{name}` not found.")]
    FunctionNotFound { name: String },
    #[error("Variable {name} does not exist. Has it been declared?")]
    VariableNotFound { name: String },

    #[error("Invalid data type. Expected `{expected}`, found `{found}`.")]
    InvalidDataType { expected: String, found: String },
}
