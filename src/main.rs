use std::{env, fs, process::exit};

use chumsky::Parser;
use data::Data;
use expr::{ExecutionState, Expr};
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

pub fn execute_block(block: &[Expr], state: &ExecutionState) -> Data {
    let mut inner_state = state.clone();
    let mut output = Data::Null;

    for e in block {
        match e.eval(&mut inner_state) {
            Ok(result) => {
                println!("{} = {}", e, result);
                output = result;
            }
            Err(e) => {
                println!("{}", e);
                exit(3);
            }
        }
    }

    output
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

    let exec_state = ExecutionState::new();
    let output = execute_block(&expressions, &exec_state);

    (tokens, expressions, output)
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
