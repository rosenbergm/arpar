use std::{collections::linked_list::Iter, collections::HashMap, iter::Peekable};

use crate::lexer::{Lexer, Token, Tokens};

pub trait Parse {
    fn run(tokens: &Tokens) -> Result<Expression, String>;
}

pub struct Parser;

impl Parse for Parser {
    fn run(tokens: &Tokens) -> Result<Expression, String> {
        let mut peekable_tokens = tokens.iter().peekable();

        parse_expression(&mut peekable_tokens)
    }
}

fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    let next = tokens.next().unwrap();

    match next {
        Token::Number(number) => Ok(Expression::Number(*number)),
        Token::Variable(var) => Ok(Expression::Variable(*var)),
        Token::LeftParen => {
            let expression = parse_expression(tokens)?;
            assert_next(tokens, Token::RightParen)?;
            Ok(expression)
        }
        _ => Err(format!("Unexpected token {:?}", next)),
    }
}

fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    let expression = parse_primary(tokens)?;
    let next = tokens.peek().unwrap();

    if *next == &Token::Product {
        tokens.next();

        let right = parse_term(tokens)?;

        return Ok(Expression::Binary(
            Operator::Product,
            Box::new(expression),
            Box::new(right),
        ));
    }

    Ok(expression)
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    let expression = parse_term(tokens)?;

    let next = tokens.peek().unwrap();

    match next {
        Token::Sum => {
            tokens.next().unwrap();
            let right = parse_expression(tokens)?;
            Ok(Expression::Binary(
                Operator::Sum,
                Box::new(expression),
                Box::new(right),
            ))
        }
        _ => Ok(expression),
    }
}

fn assert_next(tokens: &mut Peekable<Iter<Token>>, token: Token) -> Result<(), String> {
    let next = tokens.next();
    if next.is_none() {
        return Err("Unexpected EOF".to_string());
    }

    if *next.unwrap() != token {
        return Err(format!("Expected {:?} actual {:?}", token, next.unwrap(),));
    }

    Ok(())
}

#[derive(Debug)]
pub enum Operator {
    Sum,
    Product,
}

// AST Definition
#[derive(Debug)]
pub enum Expression {
    Number(u32),
    Variable(char),
    Binary(Operator, Box<Expression>, Box<Expression>),
}

// Evalutation
impl Expression {
    pub fn evaluate(&self, memory: &HashMap<String, String>) -> Result<u32, String> {
        match self {
            Expression::Number(number) => Ok(*number),
            Expression::Variable(var) => match memory.get(&var.to_string()) {
                None => Err(format!("Variable {} not found.", var)),
                Some(string_expr) => {
                    let tokens = Lexer::run(string_expr.clone())?;
                    let expression = Parser::run(&tokens)?;

                    expression.evaluate(memory)
                }
            },
            Expression::Binary(Operator::Sum, left, right) => {
                Ok(left.evaluate(memory)? + right.evaluate(memory)?)
            }
            Expression::Binary(Operator::Product, left, right) => {
                Ok(left.evaluate(memory)? * right.evaluate(memory)?)
            }
        }
    }
}
