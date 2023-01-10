use std::{collections::LinkedList, iter::Peekable};

pub type Tokens = LinkedList<Token>;

pub struct Lexer;

impl Lexer {
    pub fn run(input: String) -> Result<Tokens, String> {
        let mut result: LinkedList<Token> = LinkedList::new();

        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' => {
                    result.push_back(Token::Variable(c));
                    chars.next();
                }
                '0'..='9' => {
                    chars.next();
                    let number = get_number(c, &mut chars);
                    result.push_back(Token::Number(number))
                }
                '+' => {
                    result.push_back(Token::Sum);
                    chars.next();
                }
                '*' => {
                    result.push_back(Token::Product);
                    chars.next();
                }
                '(' => {
                    result.push_back(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    result.push_back(Token::RightParen);
                    chars.next();
                }
                ' ' | '\t' => {
                    chars.next();
                }
                _ => return Err(format!("Unexpected character `{}`", c)),
            }
        }

        result.push_back(Token::Eof);

        Ok(result)
    }
}

fn get_number<T: Iterator<Item = char>>(c: char, chars: &mut Peekable<T>) -> u32 {
    let mut number = c.to_string().parse::<u32>().expect("Expected digit.");
    while let Some(Ok(digit)) = chars.peek().map(|c| c.to_string().parse::<u32>()) {
        number = number * 10 + digit;
        chars.next();
    }
    number
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(u32),
    Variable(char),
    Sum,
    Product,
    LeftParen,
    RightParen,
    Eof,
}
