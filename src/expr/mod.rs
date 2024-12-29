use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Display, Write},
};

use error::ExprError;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{
    constants::constants,
    data::{format_types, format_vec, Data, DataType},
    execute_block,
    functions::{
        add_descriptor, and_descriptor, builtints, div_descriptor, eq_descriptor, ge_descriptor,
        gt_descriptor, le_descriptor, lt_descriptor, mod_descriptor, mul_descriptor, ne_descriptor,
        neg_descriptor, not_descriptor, or_descriptor, sub_descriptor, xor_descriptor,
        FunctionDescriptor, FunctionMap, FunctionType,
    },
    utils::strings::{indent, DotDebug, DotDisplay},
};

pub mod error;

pub type VariableMap = HashMap<String, Data>;

#[derive(Debug, Clone)]
pub struct ExecutionState {
    pub functions: FunctionMap,
    variables: VariableMap,
    constants: VariableMap,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            functions: builtints(),
            variables: HashMap::new(),
            constants: constants(),
        }
    }
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self::new()
    }
}

type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(Decimal),
    Bool(bool),
    String(String),
    Array(Vec<Expr>),
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
    FunctionDeclaration(String, FunctionDescriptor),
    VariableDeclaration(String, BExpr),
    Variable(String),

    If(BExpr, Vec<Expr>, Vec<(Expr, Vec<Expr>)>, Option<Vec<Expr>>),
    For(String, BExpr, Vec<Expr>),
    While(BExpr, Vec<Expr>),
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
            Ok(match $func.function {
                FunctionType::BuiltIn(f) => f(inputs)?,
                FunctionType::Custom(block, input_names) => {
                    let mut state = ExecutionState::new();

                    for (i, name) in input_names.iter().enumerate() {
                        state.variables.insert(name.clone(), inputs[i].clone());
                    }

                    execute_block(&block, &state).0
                }
            })
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
            Expr::String(s) => Ok(Data::String(s.clone())),
            Expr::Array(a) => Ok(Data::Array(
                a.iter()
                    .map(|e| e.eval(state))
                    .collect::<EResult<Vec<_>>>()?,
            )),

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

            Expr::Block(block) => Ok(execute_block(block, state).0),
            Expr::If(cond, if_block, elifs, else_block) => {
                let cond = cond.eval(state)?;

                if let Data::Bool(b) = cond {
                    if b {
                        Ok(execute_block(if_block, state).0)
                    } else {
                        for (cond, block) in elifs {
                            let cond = cond.eval(state)?;
                            if let Data::Bool(b) = cond {
                                if b {
                                    return Ok(execute_block(block, state).0);
                                }
                            } else {
                                return Err(ExprError::InvalidDataType {
                                    expected: "Bool".to_string(),
                                    found: cond._type().to_string(),
                                    loc: "if condition".to_string(),
                                });
                            }
                        }

                        if let Some(block) = else_block {
                            Ok(execute_block(block, state).0)
                        } else {
                            Ok(Data::Null)
                        }
                    }
                } else {
                    Err(ExprError::InvalidDataType {
                        expected: "Bool".to_string(),
                        found: cond._type().to_string(),
                        loc: "if condition".to_string(),
                    })
                }
            }

            Expr::While(cond, block) => {
                let initial_state = state.clone();
                let mut inner_state = state.clone();

                let is_true = |data| {
                    if let Data::Bool(b) = data {
                        b
                    } else {
                        false
                    }
                };

                while is_true(cond.eval(&mut inner_state)?) {
                    let (_, s) = execute_block(block, &inner_state);
                    inner_state = s;
                }

                *state = initial_state;

                Ok(Data::Null)
            }
            Expr::For(var_name, maybe_array, block) => {
                let maybe_array = maybe_array.eval(state)?;

                if let Data::Array(array) = maybe_array {
                    for data in array {
                        let mut inner_state = state.clone();
                        inner_state.variables.insert(var_name.clone(), data);

                        execute_block(block, &inner_state);
                    }

                    Ok(Data::Null)
                } else {
                    Err(ExprError::InvalidDataType {
                        expected: "Array".to_string(),
                        found: maybe_array._type().to_string(),
                        loc: "for loop input".to_string(),
                    })
                }
            }

            Expr::Function(name, inputs) => {
                if let Some(func) = state.functions.get(name) {
                    run_fn_owned(func.clone(), inputs, state)
                } else {
                    // TODO: Allow users to define their own functions.
                    Err(ExprError::FunctionNotFound { name: name.clone() })
                }
            }
            Expr::FunctionDeclaration(_, _) => Ok(Data::Null),
            Expr::Variable(name) => {
                if let Some(v) = state.variables.get(name) {
                    Ok(v.clone())
                } else if let Some(v) = state.constants.get(name) {
                    Ok(v.clone())
                } else {
                    Err(ExprError::VariableNotFound { name: name.clone() })
                }
            }
            Expr::VariableDeclaration(name, value) => {
                let value = value.eval(state)?;

                state.variables.insert(name.clone(), value);

                Ok(Data::Null)
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
//

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => format!("{b}"),
                Self::Num(n) => format!("{n}"),
                Self::Null => "null".into(),
                Self::String(s) => format!("\"{s}\""),
                Self::Array(a) => format_vec(a),

                Self::Neg(e) => format!("(-{e})"),
                Self::Add(l, r) => format!("({l} + {r})"),
                Self::Sub(l, r) => format!("({l} - {r})"),
                Self::Mul(l, r) => format!("({l} * {r})"),
                Self::Div(l, r) => format!("({l} / {r})"),
                Self::Mod(l, r) => format!("({l} % {r})"),

                Self::Ge(l, r) => format!("({l} >= {r})"),
                Self::Gt(l, r) => format!("({l} > {r})"),
                Self::Le(l, r) => format!("({l} <= {r})"),
                Self::Lt(l, r) => format!("({l} < {r})"),
                Self::Eq(l, r) => format!("({l} == {r})"),
                Self::Ne(l, r) => format!("({l} != {r})"),

                Self::And(l, r) => format!("({l} && {r})"),
                Self::Or(l, r) => format!("({l} || {r})"),
                Self::Xor(l, r) => format!("({l} ^ {r})"),
                Self::Not(e) => format!("!{e}"),

                Self::While(cond, block) => format!("while {cond} {}", format_block(block)),
                Self::For(name, array, block) =>
                    format!("for {name} in {array} {}", format_block(block)),

                Self::If(cond, if_block, elif_blocks, else_block) => format!(
                    "if ({cond}) {} {} {}",
                    format_block(if_block),
                    elif_blocks
                        .iter()
                        .map(|(cond, block)| format!("elif ({}) {}", cond, format_block(block)))
                        .collect::<String>(),
                    if let Some(block) = else_block {
                        format!("else {}", format_block(block))
                    } else {
                        String::new()
                    }
                ),

                Self::Block(exps) => format_block(exps),

                Self::Variable(name) => name.to_string(),
                Self::VariableDeclaration(name, value) => format!("let {name} := {value}"),
                Self::Function(name, inputs) => {
                    let mut s = name.clone();

                    write!(s, "(").unwrap();

                    for (i, arg) in inputs.iter().enumerate() {
                        if i != inputs.len() - 1 {
                            write!(s, "{}, ", arg).unwrap();
                        } else {
                            write!(s, "{arg}").unwrap();
                        }
                    }

                    write!(s, ")").unwrap();

                    s
                }

                Self::FunctionDeclaration(name, block) => {
                    todo!()
                }
            }
        )
    }
}

impl Expr {
    pub fn data_type(&self, state: &ExecutionState) -> DataType {
        match self {
            Expr::String(_) => DataType::String,
            Expr::Add(_, _)
            | Expr::Sub(_, _)
            | Expr::Mul(_, _)
            | Expr::Div(_, _)
            | Expr::Mod(_, _)
            | Expr::Neg(_)
            | Expr::Num(_) => DataType::Number,
            Expr::Bool(_)
            | Expr::Or(_, _)
            | Expr::And(_, _)
            | Expr::Not(_)
            | Expr::Xor(_, _)
            | Expr::Lt(_, _)
            | Expr::Le(_, _)
            | Expr::Gt(_, _)
            | Expr::Ge(_, _)
            | Expr::Eq(_, _)
            | Expr::Ne(_, _) => DataType::Bool,
            Expr::Null => DataType::Null,
            Expr::Variable(_) => DataType::Any,
            Expr::Function(name, _) => state.functions.get(name).map(|f| f.output).unwrap(),
            Expr::FunctionDeclaration(_, _) => DataType::Null,
            Expr::Array(_) => DataType::Array,
            Expr::Block(block) => block.last().unwrap().data_type(state),
            Expr::VariableDeclaration(_, _) => DataType::Null,
            Expr::If(_, b, _, _) => b.last().unwrap().data_type(state),
            Expr::For(_, _, _) | Expr::While(_, _) => DataType::Null,
        }
    }
}

pub fn format_block(block: &[Expr]) -> String {
    format!(
        "{{\n{}}}",
        indent(
            &block
                .iter()
                .map(|e| e.display())
                .collect::<Vec<String>>()
                .join("\n")
        )
    )
}
