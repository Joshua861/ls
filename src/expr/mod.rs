use std::{borrow::Borrow, collections::HashMap};

use error::ExprError;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{
    data::{format_types, Data},
    execute_block,
    functions::{
        add_descriptor, and_descriptor, builtints, div_descriptor, eq_descriptor, ge_descriptor,
        gt_descriptor, le_descriptor, lt_descriptor, mod_descriptor, mul_descriptor, ne_descriptor,
        neg_descriptor, not_descriptor, or_descriptor, sub_descriptor, xor_descriptor,
        FunctionDescriptor, FunctionMap,
    },
    utils::strings::DotDebug,
};

pub mod error;

#[derive(Debug, Clone)]
pub struct ExecutionState {
    builtins: FunctionMap,
    variables: HashMap<String, Data>,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            builtins: builtints(),
            variables: HashMap::new(),
        }
    }
}

type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(Decimal),
    Bool(bool),
    Null,

    Neg(BExpr),

    Add(BExpr, BExpr),
    Sub(BExpr, BExpr),
    Mul(BExpr, BExpr),
    Div(BExpr, BExpr),
    Mod(BExpr, BExpr),

    Gt(BExpr, BExpr),
    Lt(BExpr, BExpr),
    Ge(BExpr, BExpr),
    Le(BExpr, BExpr),
    Eq(BExpr, BExpr),
    Ne(BExpr, BExpr),
    Not(BExpr),
    And(BExpr, BExpr),
    Or(BExpr, BExpr),
    Xor(BExpr, BExpr),

    Block(Vec<Expr>),

    Function(String, Vec<Expr>),
    VariableDeclaration(String, BExpr),
    Variable(String),
}

pub type EResult<T> = Result<T, ExprError>;

macro_rules! run {
    ($func: ident, $inputs: ident, $state: ident) => {{
        let inputs = $inputs
            .iter()
            .map(|e| e.eval($state))
            .collect::<Result<Vec<_>, _>>()?;

        let matching_types = inputs
            .iter()
            .map(|i| i._type())
            .zip($func.inputs.iter())
            .all(|(input, expected)| input == *expected || expected.is_any());

        if matching_types {
            Ok(($func.function)(inputs))?
        } else {
            let input_types = inputs.iter().map(|i| i._type()).collect::<Vec<_>>();
            Err(ExprError::InvalidFunctionArguements {
                expected: format_types($func.inputs),
                found: format_types(input_types),
                // name:  name.clone(),
            })
        }
    }};
}

fn run_fn(
    func: FunctionDescriptor,
    inputs: &[&BExpr],
    state: &mut ExecutionState,
) -> EResult<Data> {
    run!(func, inputs, state)
}

fn run_fn_owned(
    func: FunctionDescriptor,
    inputs: &[Expr],
    state: &mut ExecutionState,
) -> EResult<Data> {
    run!(func, inputs, state)
}

impl Expr {
    pub fn eval(&self, state: &mut ExecutionState) -> EResult<Data> {
        match self {
            Expr::Num(n) => Ok(Data::Number(*n)),
            Expr::Bool(b) => Ok(Data::Bool(*b)),
            Expr::Null => Ok(Data::Null),

            Expr::Neg(n) => run_fn(neg_descriptor(), &[n], state),
            Expr::Add(lhs, rhs) => run_fn(add_descriptor(), &[lhs, rhs], state),
            Expr::Sub(lhs, rhs) => run_fn(sub_descriptor(), &[lhs, rhs], state),
            Expr::Mul(lhs, rhs) => run_fn(mul_descriptor(), &[lhs, rhs], state),
            Expr::Div(lhs, rhs) => run_fn(div_descriptor(), &[lhs, rhs], state),
            Expr::Mod(lhs, rhs) => run_fn(mod_descriptor(), &[lhs, rhs], state),

            Expr::Le(lhs, rhs) => run_fn(le_descriptor(), &[lhs, rhs], state),
            Expr::Gt(lhs, rhs) => run_fn(gt_descriptor(), &[lhs, rhs], state),
            Expr::Ge(lhs, rhs) => run_fn(ge_descriptor(), &[lhs, rhs], state),
            Expr::Lt(lhs, rhs) => run_fn(lt_descriptor(), &[lhs, rhs], state),
            Expr::Eq(lhs, rhs) => run_fn(eq_descriptor(), &[lhs, rhs], state),
            Expr::Ne(lhs, rhs) => run_fn(ne_descriptor(), &[lhs, rhs], state),

            Expr::Not(n) => run_fn(not_descriptor(), &[n], state),
            Expr::And(lhs, rhs) => run_fn(and_descriptor(), &[lhs, rhs], state),
            Expr::Or(lhs, rhs) => run_fn(or_descriptor(), &[lhs, rhs], state),
            Expr::Xor(lhs, rhs) => run_fn(xor_descriptor(), &[lhs, rhs], state),

            Expr::Block(block) => Ok(execute_block(block, &state)),

            Expr::Function(name, inputs) => {
                if let Some(func) = state.builtins.get(name) {
                    run_fn_owned(func.clone(), inputs, state)
                } else {
                    // TODO: Allow users to define their own functions.
                    Err(ExprError::FunctionNotFound { name: name.clone() })
                }
            }
            Expr::Variable(name) => state
                .variables
                .get(name)
                .ok_or(ExprError::VariableNotFound { name: name.clone() })
                .copied(),
            Expr::VariableDeclaration(name, value) => {
                let value = value.eval(state)?;

                state.variables.insert(name.clone(), value);

                Ok(value)
            }
        }
    }
}

// pub trait Boxed {
//     fn boxed(&self) -> Box<Self>;
// }
//
// impl<T> Boxed for T
// where
//     T: Clone,
// {
//     fn boxed(&self) -> Box<Self> {
//         Box::new(self.clone())
//     }
// }
