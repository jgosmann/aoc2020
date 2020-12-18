use std::fmt::Display;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::num::ParseIntError;

type ValueType = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Num(ValueType),
    Operator(Operator),
    OpenParens,
    CloseParens,
}

#[derive(Clone, Debug, PartialEq)]
enum Ast {
    Leaf(ValueType),
    Node(Box<Ast>, Operator, Box<Ast>),
}

impl Ast {
    fn evaluate(&self) -> ValueType {
        match self {
            Self::Leaf(v) => *v,
            Self::Node(lhs, op, rhs) => match op {
                Operator::Add => lhs.evaluate() + rhs.evaluate(),
                Operator::Multiply => lhs.evaluate() * rhs.evaluate(),
            },
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Leaf(v) => f.write_fmt(format_args!("{}", v)),
            Self::Node(lhs, Operator::Add, rhs) => f.write_fmt(format_args!("{} + {}", lhs, rhs)),
            Self::Node(lhs, Operator::Multiply, rhs) => {
                f.write_fmt(format_args!("({} * {})", lhs, rhs))
            }
        }
    }
}

struct StripWhitespace<I: Iterator<Item = char>> {
    iterator: I,
}

impl<I> Iterator for StripWhitespace<I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.iterator.next() {
            if !c.is_whitespace() {
                return Some(c);
            }
        }
        None
    }
}

struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<StripWhitespace<I>>,
}

impl<'a> Tokenizer<std::str::Chars<'a>> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: StripWhitespace {
                iterator: input.chars(),
            }
            .peekable(),
        }
    }
}

impl<I> Iterator for Tokenizer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, ParseIntError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some('(') => Some(Ok(Token::OpenParens)),
            Some(')') => Some(Ok(Token::CloseParens)),
            Some('+') => Some(Ok(Token::Operator(Operator::Add))),
            Some('*') => Some(Ok(Token::Operator(Operator::Multiply))),
            None => None,
            Some(c) => {
                let mut buf = String::from(c);
                while let Some(c) = self.chars.peek() {
                    if !c.is_digit(10) {
                        break;
                    }
                    buf.push(self.chars.next().unwrap());
                }
                Some(buf.parse::<ValueType>().map(|v| Token::Num(v)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum PartialParse {
    Token(Token),
    Node(Ast),
    None,
}

impl Ast {
    pub fn parse(tokens: &mut impl Iterator<Item = Token>) -> Result<Self, ()> {
        let mut stack = vec![];
        let mut tokens = tokens.peekable();
        while let Some(token) = tokens.next() {
            stack.push(PartialParse::Token(token));
            loop {
                match (
                    if stack.len() >= 3 {
                        stack[stack.len() - 3].clone()
                    } else {
                        PartialParse::None
                    },
                    if stack.len() >= 2 {
                        stack[stack.len() - 2].clone()
                    } else {
                        PartialParse::None
                    },
                    if stack.len() >= 1 {
                        stack[stack.len() - 1].clone()
                    } else {
                        PartialParse::None
                    },
                    tokens.peek(),
                ) {
                    (_, _, PartialParse::Token(Token::Num(v)), _) => {
                        stack.pop();
                        stack.push(PartialParse::Node(Ast::Leaf(v)))
                    }
                    (
                        PartialParse::Token(Token::OpenParens),
                        PartialParse::Node(x),
                        PartialParse::Token(Token::CloseParens),
                        _,
                    ) => {
                        (0..3).for_each(|_| {
                            stack.pop();
                        });
                        stack.push(PartialParse::Node(x));
                    }
                    (
                        PartialParse::Node(lhs),
                        PartialParse::Token(Token::Operator(Operator::Add)),
                        PartialParse::Node(rhs),
                        _,
                    ) => {
                        (0..3).for_each(|_| {
                            stack.pop();
                        });
                        stack.push(PartialParse::Node(Ast::Node(
                            Box::new(lhs),
                            Operator::Add,
                            Box::new(rhs),
                        )));
                    }
                    (
                        PartialParse::Node(lhs),
                        PartialParse::Token(Token::Operator(Operator::Multiply)),
                        PartialParse::Node(rhs),
                        Some(Token::CloseParens),
                    ) => {
                        (0..3).for_each(|_| {
                            stack.pop();
                        });
                        stack.push(PartialParse::Node(Ast::Node(
                            Box::new(lhs),
                            Operator::Multiply,
                            Box::new(rhs),
                        )));
                    }
                    (
                        PartialParse::Node(lhs),
                        PartialParse::Token(Token::Operator(Operator::Multiply)),
                        PartialParse::Node(rhs),
                        Some(Token::Operator(Operator::Multiply)),
                    ) => {
                        (0..3).for_each(|_| {
                            stack.pop();
                        });
                        stack.push(PartialParse::Node(Ast::Node(
                            Box::new(lhs),
                            Operator::Multiply,
                            Box::new(rhs),
                        )));
                    }
                    (
                        PartialParse::Node(lhs),
                        PartialParse::Token(Token::Operator(Operator::Multiply)),
                        PartialParse::Node(rhs),
                        None,
                    ) => {
                        (0..3).for_each(|_| {
                            stack.pop();
                        });
                        stack.push(PartialParse::Node(Ast::Node(
                            Box::new(lhs),
                            Operator::Multiply,
                            Box::new(rhs),
                        )));
                    }
                    _ => break,
                }
            }
        }

        if stack.len() != 1 {
            return Err(());
        }
        if let PartialParse::Node(node) = stack[0].clone() {
            return Ok(node);
        }
        Err(())
    }
}

fn main() {
    let stdin = io::stdin();
    let result: ValueType = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let tokens: Result<Vec<Token>, ParseIntError> = Tokenizer::new(&line).collect();
            let tokens = tokens.unwrap();
            let ast = Ast::parse(&mut tokens.iter().copied()).unwrap();
            ast.evaluate()
        })
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let tokens: Result<Vec<Token>, ParseIntError> = Tokenizer::new("1 + (2 * 3)").collect();
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token::Num(1),
                Token::Operator(Operator::Add),
                Token::OpenParens,
                Token::Num(2),
                Token::Operator(Operator::Multiply),
                Token::Num(3),
                Token::CloseParens
            ]
        )
    }

    #[test]
    fn test_parsing_and_evaluation() {
        let tokens: Result<Vec<Token>, ParseIntError> =
            Tokenizer::new("1 + (2 * 3) + (4 * (5 + 6))").collect();
        let tokens = tokens.unwrap();
        let ast = Ast::parse(&mut tokens.iter().copied()).unwrap();
        assert_eq!(ast.evaluate(), 51);
    }

    #[test]
    fn test_parsing_and_evaluation2() {
        let tokens: Result<Vec<Token>, ParseIntError> =
            Tokenizer::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").collect();
        let tokens = tokens.unwrap();
        let ast = Ast::parse(&mut tokens.iter().copied()).unwrap();
        assert_eq!(ast.evaluate(), 669060);
    }
}
