use nom::{
    branch::alt,
    character::complete::{alphanumeric1, char, digit1, space0, space1},
    combinator::map,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum ProductionRule {
    Terminal(String),
    OneOf(Vec<Box<ProductionRule>>),
    Sequence(Vec<Box<ProductionRule>>),
    Ref(usize),
}

#[derive(Debug, PartialEq)]
struct Grammar {
    rules: HashMap<usize, ProductionRule>,
    root: usize,
}

impl ProductionRule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let terminal = map(delimited(char('"'), alphanumeric1, char('"')), |token| {
            Self::Terminal(String::from(token))
        });
        let reference = map(map_res(digit1, |num: &str| num.parse()), |token| {
            Self::Ref(token)
        });
        let sequence = map(
            separated_list1(space1, alt((terminal, reference))),
            |tokens| Self::Sequence(tokens.into_iter().map(Box::new).collect()),
        );
        map(
            separated_list1(tuple((space0, char('|'), space0)), sequence),
            |tokens| Self::OneOf(tokens.into_iter().map(Box::new).collect()),
        )(input)
    }
}

#[derive(Debug)]
enum GrammarParseError {
    RuleParseError,
    ExtraCharacters,
}

impl Display for GrammarParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::RuleParseError => f.write_str("Error while parsing rule.")?,
            Self::ExtraCharacters => f.write_str("Extra characters after rule.")?,
        }
        Ok(())
    }
}

impl std::error::Error for GrammarParseError {}

impl Grammar {
    fn parse_rule(input: &str) -> IResult<&str, (usize, ProductionRule)> {
        let key = map_res(digit1, |num: &str| num.parse());
        let separator = tuple((char(':'), space0));
        separated_pair(key, separator, ProductionRule::parse)(input)
    }

    fn parse_lines(
        lines: &mut impl Iterator<Item = impl AsRef<str>>,
    ) -> Result<Self, GrammarParseError> {
        Ok(Self {
            rules: lines
                .map(|line| {
                    let (extra_chars, keyed_rule) = Self::parse_rule(line.as_ref())
                        .map_err(|err| GrammarParseError::RuleParseError)?;
                    if extra_chars.is_empty() {
                        Ok(keyed_rule)
                    } else {
                        Err(GrammarParseError::ExtraCharacters)
                    }
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            root: 0,
        })
    }

    fn rule_accepts<'a>(&self, rule: &ProductionRule, input: &'a str) -> Option<&'a str> {
        use ProductionRule::*;
        match rule {
            Terminal(terminal) => {
                if input.starts_with(terminal) {
                    Some(&input[terminal.len()..])
                } else {
                    None
                }
            }
            Sequence(children) => {
                let mut input = input;
                for child in children {
                    if let Some(remainder) = self.rule_accepts(child, input) {
                        input = remainder;
                    } else {
                        return None;
                    }
                }
                Some(input)
            }
            OneOf(children) => {
                for child in children {
                    if let Some(remainder) = self.rule_accepts(child, input) {
                        return Some(remainder);
                    }
                }
                None
            }
            Ref(referenced_rule) => self.rule_accepts(self.rules.get(referenced_rule)?, input),
        }
    }

    fn accepts(&self, input: &str) -> bool {
        if let Some(root) = self.rules.get(&self.root) {
            if let Some(remainder) = self.rule_accepts(root, input) {
                return remainder.is_empty();
            }
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let grammar =
        Grammar::parse_lines(&mut lines.by_ref().take_while(|line| !line.is_empty())).unwrap();
    let count = lines.filter(|line| grammar.accepts(line)).count();
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_productiion_rule() {
        let (_, rule) = ProductionRule::parse("1 3 | 3 \"x\"").unwrap();
        assert_eq!(
            rule,
            ProductionRule::OneOf(vec![
                Box::new(ProductionRule::Sequence(vec![
                    Box::new(ProductionRule::Ref(1)),
                    Box::new(ProductionRule::Ref(3)),
                ])),
                Box::new(ProductionRule::Sequence(vec![
                    Box::new(ProductionRule::Ref(3)),
                    Box::new(ProductionRule::Terminal("x".into())),
                ])),
            ])
        );
    }

    #[test]
    fn test_parse_grammar_rule() {
        let (_, rule) = Grammar::parse_rule("0: 3 \"b\"").unwrap();
        assert_eq!(
            rule,
            (
                0,
                ProductionRule::OneOf(vec![Box::new(ProductionRule::Sequence(vec![
                    Box::new(ProductionRule::Ref(3)),
                    Box::new(ProductionRule::Terminal("b".into()))
                ]))])
            )
        )
    }

    #[test]
    fn test_parse_grammar_lines() {
        let grammar = Grammar::parse_lines(&mut vec!["0: 1", "1: \"b\""].iter()).unwrap();
        assert_eq!(
            grammar,
            Grammar {
                root: 0,
                rules: vec![
                    (
                        0,
                        ProductionRule::OneOf(vec![Box::new(ProductionRule::Sequence(vec![
                            Box::new(ProductionRule::Ref(1)),
                        ]))])
                    ),
                    (
                        1,
                        ProductionRule::OneOf(vec![Box::new(ProductionRule::Sequence(vec![
                            Box::new(ProductionRule::Terminal("b".into())),
                        ]))])
                    )
                ]
                .into_iter()
                .collect()
            }
        );
    }

    #[test]
    fn test_grammar_accepts() {
        let grammar = Grammar::parse_lines(
            &mut vec!["0: 1 2", "1: \"a\"", "2: 1 3 | 3 1", "3: \"b\""].iter(),
        )
        .unwrap();
        assert_eq!(grammar.accepts("aab"), true);
        assert_eq!(grammar.accepts("aba"), true);
        assert_eq!(grammar.accepts("aaa"), false);
        assert_eq!(grammar.accepts("baa"), false);
        assert_eq!(grammar.accepts("bab"), false);
        assert_eq!(grammar.accepts("bba"), false);
        assert_eq!(grammar.accepts("bbb"), false);
    }
}
