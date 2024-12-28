use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    error::Simple,
    prelude::{end, just, recursive},
    select, Parser,
};

use crate::{
    expr::Expr,
    lexer::Token,
    utils::strings::{DotDebug, DotDisplay},
};

pub fn parser() -> impl Parser<Token, Vec<Expr>, Error = Simple<Token>> {
    let statement = recursive(|stmt| {
        let expr = recursive(|p| {
            let parenthesized = p
                .clone()
                .delimited_by(just(Token::LParen), just(Token::RParen));

            let integer = select! {
                Token::Number(n) => Expr::Num(n),
            };

            let negative_integer = just(Token::Minus)
                .then(integer)
                .map(|(_minus, expr)| Expr::Neg(Box::new(expr)));

            let bool = select! {
                Token::True => Expr::Bool(true),
                Token::False => Expr::Bool(false),
            };

            let string = select! {
                Token::String(s) => Expr::String(s),
            };

            let variable = select! {
                Token::Ident(name) => Expr::Variable(name),
            };

            let function = select! {
                Token::Ident(name) => name,
            }
            .then(
                p.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .map(|(name, args)| Expr::Function(name, args));

            let block = just(Token::BlockStart)
                .ignore_then(stmt.clone().repeated())
                .then_ignore(just(Token::BlockEnd));

            let if_block = just(Token::If)
                .then(parenthesized.clone())
                .then(block.clone())
                .then(
                    just(Token::ElseIf)
                        .then(parenthesized.clone())
                        .then(block.clone())
                        .repeated(),
                )
                .then(just(Token::Else).then(block.clone()).or_not())
                .map(|((((_, cond), if_block), elifs), else_block)| {
                    Expr::If(
                        Box::new(cond),
                        if_block,
                        elifs
                            .iter()
                            .map(|((_, cond), block)| (cond.clone(), block.clone()))
                            .collect::<Vec<_>>(),
                        else_block.map(|(_, block)| block),
                    )
                });

            let block = block.map(Expr::Block);

            let array = p
                .clone()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .delimited_by(just(Token::ArrayStart), just(Token::ArrayEnd))
                .map(Expr::Array);

            let atom = block
                .or(parenthesized)
                .or(integer)
                .or(negative_integer)
                .or(bool)
                .or(function.clone())
                .or(variable)
                .or(if_block)
                .or(array)
                .or(string);

            let atom = atom
                .clone()
                .then(just(Token::Dot).ignore_then(function).repeated())
                .map(|(initial, method_calls)| {
                    method_calls
                        .into_iter()
                        .fold(initial, |acc, method| match method {
                            Expr::Function(name, mut args) => {
                                let mut new_args = vec![acc];
                                new_args.append(&mut args);
                                Expr::Function(name, new_args)
                            }
                            _ => unreachable!(),
                        })
                });

            let not = just(Token::Not)
                .then(atom.clone())
                .map(|(_, expr)| Expr::Not(Box::new(expr)));
            let unary = just(Token::Minus)
                .repeated()
                .then(atom.or(not))
                .foldr(|_op, lhs| Expr::Neg(Box::new(lhs)));

            let binary_1 = unary
                .clone()
                .then(
                    just(Token::Multiply)
                        .or(just(Token::Divide))
                        .or(just(Token::Modulo))
                        .then(unary)
                        .repeated(),
                )
                .foldl(|rhs, (op, lhs)| match op {
                    Token::Multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                    Token::Divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
                    Token::Modulo => Expr::Mod(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                });

            let binary_2 = binary_1
                .clone()
                .then(
                    just(Token::Plus)
                        .or(just(Token::Minus))
                        .then(binary_1)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| match op {
                    Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                    Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                });

            let boolean_1 = binary_2
                .clone()
                .then(
                    just(Token::GreaterEqual)
                        .or(just(Token::GreaterThan))
                        .or(just(Token::LessEqual))
                        .or(just(Token::LessThan))
                        .or(just(Token::Equals))
                        .or(just(Token::NotEquals))
                        .then(binary_2.clone())
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| match op {
                    Token::GreaterEqual => Expr::Ge(Box::new(lhs), Box::new(rhs)),
                    Token::GreaterThan => Expr::Gt(Box::new(lhs), Box::new(rhs)),
                    Token::LessEqual => Expr::Le(Box::new(lhs), Box::new(rhs)),
                    Token::LessThan => Expr::Lt(Box::new(lhs), Box::new(rhs)),
                    Token::Equals => Expr::Eq(Box::new(lhs), Box::new(rhs)),
                    Token::NotEquals => Expr::Ne(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                });

            let boolean_2 = boolean_1
                .clone()
                .then(
                    just(Token::And)
                        .or(just(Token::Or))
                        .or(just(Token::Xor))
                        .then(boolean_1)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| match op {
                    Token::And => Expr::And(Box::new(lhs), Box::new(rhs)),
                    Token::Or => Expr::Or(Box::new(lhs), Box::new(rhs)),
                    Token::Xor => Expr::Xor(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                });

            #[allow(clippy::let_and_return)]
            boolean_2
        });

        let variable_declaration = just(Token::Let)
            .then(select! { Token::Ident(k) => k })
            .then_ignore(just(Token::AssignTo))
            .then(expr.clone())
            .map(|((_, name), value)| Expr::VariableDeclaration(name, Box::new(value)))
            .then_ignore(just(Token::Eol));

        variable_declaration.or(expr.clone().then_ignore(just(Token::Eol)))
    });

    statement.repeated().then_ignore(end())
}

pub fn print_parser_error(err: Simple<Token>, source: &[Token]) {
    Report::build(ReportKind::Error, err.span())
        .with_code(3)
        .with_message(err.to_string())
        .with_label(
            Label::new(err.span())
                .with_message(err.reason().debug())
                .with_color(Color::Red),
        )
        .finish()
        .print(Source::from(
            source
                .iter()
                .map(|i| i.display().chars().next().unwrap())
                .collect::<String>(),
        ))
        .unwrap();
}
