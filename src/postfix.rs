use std::collections::LinkedList;

use crate::{
    lexer::{Token, Tokens},
    parser::{Expression, Operator, Parse},
};

pub struct PostfixParser;

impl Parse for PostfixParser {
    fn run(tokens: &Tokens) -> Result<Expression, String> {
        let mut tokens = tokens.clone();

        match tokens.pop_back() {
            None => Err("Unexpected error".to_string()),
            Some(Token::Eof) => parse_expression(&mut tokens),
            Some(_) => Err("Parsing error".to_string()),
        }
    }
}

fn parse_expression(tokens: &mut LinkedList<Token>) -> Result<Expression, String> {
    let this = tokens.pop_back().unwrap();

    match this {
        Token::Sum => {
            let right = parse_expression(tokens)?;
            let left = parse_expression(tokens)?;

            Ok(Expression::Binary(
                Operator::Sum,
                Box::new(left),
                Box::new(right),
            ))
        }
        Token::Product => {
            let right = parse_expression(tokens)?;
            let left = parse_expression(tokens)?;

            Ok(Expression::Binary(
                Operator::Product,
                Box::new(left),
                Box::new(right),
            ))
        }
        Token::Number(a) => Ok(Expression::Number(a)),
        Token::Variable(var) => Ok(Expression::Variable(var)),
        Token::LeftParen | Token::RightParen => {
            Err("Parenthesis are not allowed in postfix notation.".to_string())
        }
        _ => Err("Unexpected error".to_string()),
    }
}
