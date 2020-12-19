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

#[derive(Clone, Debug, PartialEq)]
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

    fn rule_accepts<'a>(&self, rule: &ProductionRule, inputs: &Vec<&'a str>) -> Vec<&'a str> {
        use ProductionRule::*;
        inputs
            .iter()
            .flat_map(|input| match rule {
                Terminal(terminal) => {
                    if input.starts_with(terminal) {
                        vec![&input[terminal.len()..]]
                    } else {
                        vec![]
                    }
                }
                Sequence(children) => {
                    if children.len() == 1 {
                        self.rule_accepts(&children[0], &vec![input])
                    } else {
                        self.rule_accepts(&children[0], &vec![input])
                            .iter()
                            .flat_map(|remainder| {
                                self.rule_accepts(
                                    &ProductionRule::Sequence(
                                        children.iter().skip(1).cloned().collect(),
                                    ),
                                    &vec![remainder],
                                )
                            })
                            .collect()
                    }
                }
                OneOf(children) => children
                    .iter()
                    .flat_map(|child| self.rule_accepts(child, &vec![input]))
                    .collect(),
                Ref(referenced_rule) => {
                    if let Some(child_rule) = self.rules.get(referenced_rule) {
                        self.rule_accepts(child_rule, &vec![input])
                    } else {
                        vec![]
                    }
                }
            })
            .collect()
    }

    fn accepts(&self, input: &str) -> bool {
        if let Some(root) = self.rules.get(&self.root) {
            for remainder in self.rule_accepts(root, &vec![input]) {
                if remainder.is_empty() {
                    return true;
                }
            }
        }
        false
    }
}

fn process(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut lines = lines;
    let mut grammar =
        Grammar::parse_lines(&mut lines.by_ref().take_while(|line| !line.as_ref().is_empty()))
            .unwrap();
    grammar.rules.insert(
        8,
        ProductionRule::OneOf(vec![
            Box::new(ProductionRule::Sequence(vec![
                Box::new(ProductionRule::Ref(42)),
                Box::new(ProductionRule::Ref(8)),
            ])),
            Box::new(ProductionRule::Ref(42)),
        ]),
    );
    grammar.rules.insert(
        11,
        ProductionRule::OneOf(vec![
            Box::new(ProductionRule::Sequence(vec![
                Box::new(ProductionRule::Ref(42)),
                Box::new(ProductionRule::Ref(11)),
                Box::new(ProductionRule::Ref(31)),
            ])),
            Box::new(ProductionRule::Sequence(vec![
                Box::new(ProductionRule::Ref(42)),
                Box::new(ProductionRule::Ref(31)),
            ])),
        ]),
    );
    lines.filter(|line| grammar.accepts(line.as_ref())).count()
}

fn main() {
    let stdin = io::stdin();
    let count = process(stdin.lock().lines().map(Result::unwrap));
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

    #[test]
    fn test_process() {
        let input = vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        assert_eq!(process(input.iter()), 12);
    }
}
