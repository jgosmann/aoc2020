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

fn process(tokens: impl Iterator<Item = Token>) -> Result<ValueType, ()> {
    let mut stack = vec![];
    for token in tokens {
        stack.push(token);
        loop {
            if stack.len() < 3 {
                break;
            }
            match (
                stack[stack.len() - 3],
                stack[stack.len() - 2],
                stack[stack.len() - 1],
            ) {
                (Token::Num(x), Token::Operator(op), Token::Num(y)) => {
                    (0..3).for_each(|_| {
                        stack.pop();
                    });
                    stack.push(Token::Num(match op {
                        Operator::Add => x + y,
                        Operator::Multiply => x * y,
                    }));
                }
                (Token::OpenParens, x, Token::CloseParens) => {
                    (0..3).for_each(|_| {
                        stack.pop();
                    });
                    stack.push(x);
                }
                _ => break,
            }
        }
    }

    if stack.len() != 1 {
        return Err(());
    }
    if let Token::Num(x) = stack[0] {
        return Ok(x);
    }
    Err(())
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
            process(tokens.iter().copied()).unwrap()
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
    fn test_processing() {
        let tokens: Result<Vec<Token>, ParseIntError> =
            Tokenizer::new("1 + (2 * 3) + (4 * (5 + 6))").collect();
        let tokens = tokens.unwrap();
        assert_eq!(process(tokens.iter().copied()).unwrap(), 51);
    }
}
