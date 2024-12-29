use std::{env, fs, process::exit};

use chumsky::Parser;
use data::{Data, DataType};
use expr::{format_block, ExecutionState, Expr};
use lexer::Token;
use logos::Logos;
use parser::{parser, print_parser_error};
use utils::strings::{indent, DotDebug};

mod data;
mod expr;
mod functions;
mod lexer;
// rip
// mod number;
mod constants;
mod parser;
mod utils;

const HELP: &str = r#"Command line calculator.

Commands:
    calc: evaluate the second arguement (string)
    run: run script from file path (string)
"#;

fn main() {
    match env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("{}", HELP);
            exit(1)
        })
        .as_str()
    {
        "calc" => {
            run(&env::args().nth(2).unwrap_or_else(|| {
                println!("Expected expression as second arguement (e.g. `1 + 7 * (3 - 4) / 5`)");
                exit(1);
            }));
        }
        "run" => {
            let path = env::args().nth(2).unwrap_or_else(|| {
                println!("Expected file path as second argument.");
                exit(1)
            });
            let text = fs::read_to_string(path).unwrap_or_else(|e| {
                println!("Could not read file: {e}");
                exit(1)
            });

            run(&text);
        }
        _ => {
            println!("Invalid command.\n");

            println!("{}", HELP);
        }
    }
}

pub fn execute_block(block: &[Expr], state: &ExecutionState) -> (Data, ExecutionState) {
    let mut inner_state = state.clone();
    let mut output = Data::Null;

    for e in block {
        if let Expr::FunctionDeclaration(name, desc) = e {
            match desc.function.clone() {
                functions::FunctionType::Custom(block, _) => {
                    let dt = block.last().unwrap_or(&Expr::Null).data_type(state);

                    if dt != desc.output && dt != DataType::Any && desc.output != DataType::Any {
                        println!("Function `{name}` output type does not match block data type. If you don't know what the output will be, you can use the Any type.");
                        exit(1);
                    }
                }
                _ => unreachable!(),
            }
            inner_state.functions.insert(name.clone(), desc.clone());
        }
    }

    for e in block {
        match e.eval(&mut inner_state) {
            Ok(result) => {
                // println!("{}", e);
                output = result;
            }
            Err(e) => {
                println!("{}", e);
                exit(3);
            }
        }
    }

    (output, inner_state)
}

fn run(input: &str) -> (Vec<Token>, Vec<Expr>, Data) {
    let lexer = Token::lexer(input);

    let mut tokens = vec![];
    for (token, _span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
            Err(e) => {
                e.print(input);
                exit(1);
            }
        }
    }

    let tokens = tokens
        .iter()
        .filter(|t| !t.is_comment())
        .cloned()
        .collect::<Vec<_>>();

    let expressions = match parser().parse(tokens.clone()) {
        Ok(expr) => {
            println!("[AST]\n{}", indent(&expr.debug()));
            expr
        }
        Err(errs) => {
            for err in errs {
                print_parser_error(err, &tokens);
            }
            exit(1);
        }
    };

    // println!("{}", format_block(&expressions));

    println!("\n---Execution---\n");

    let exec_state = ExecutionState::new();
    let output = execute_block(&expressions, &exec_state);

    (tokens, expressions, output.0)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    fn test_num(input: &str, expected: Decimal) {
        let (_, _, output) = run(input);
        assert_eq!(output, Data::Number(expected))
    }

    #[test]
    fn lex_num() {
        let (tokens, _, _) = run("5.5;");

        assert_eq!(
            tokens,
            vec![Token::Number(Decimal::from_f64(5.5).unwrap()), Token::Eol]
        )
    }

    #[test]
    fn test_order() {
        test_num("2-5*2+7;", dec!(-1));
    }
}
