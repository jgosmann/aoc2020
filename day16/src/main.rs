use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, line_ending, space1},
    combinator::{eof, map, map_res, recognize},
    multi::{many1, separated_list0},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::io::{self, Read};

mod graph;

use graph::DirectedGraph;

trait Parsable<T> {
    fn parse(input: &str) -> IResult<&str, T>;
}

type Value = u64;

impl Parsable<Value> for Value {
    fn parse(input: &str) -> IResult<&str, Value> {
        map_res(digit1, str::parse::<Value>)(input)
    }
}

#[derive(Debug, PartialEq)]
struct InclusiveRange(Value, Value);

impl Parsable<InclusiveRange> for InclusiveRange {
    fn parse(input: &str) -> IResult<&str, InclusiveRange> {
        map(
            separated_pair(Value::parse, char('-'), Value::parse),
            |(lb, ub)| InclusiveRange(lb, ub),
        )(input)
    }
}

impl From<(Value, Value)> for InclusiveRange {
    fn from(value: (Value, Value)) -> Self {
        InclusiveRange(value.0, value.1)
    }
}

impl InclusiveRange {
    fn contains(&self, value: &Value) -> bool {
        self.0 <= *value && *value <= self.1
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    field: String,
    valid_ranges: Vec<InclusiveRange>,
}

impl Parsable<Rule> for Rule {
    fn parse(input: &str) -> IResult<&str, Rule> {
        let valid_ranges_def = separated_list0(tag(" or "), InclusiveRange::parse);
        map(
            separated_pair(
                recognize(many1(alt((alphanumeric1, space1)))),
                tag(": "),
                valid_ranges_def,
            ),
            |(field, valid_ranges)| Rule {
                field: field.into(),
                valid_ranges,
            },
        )(input)
    }
}

impl Rule {
    fn is_valid(&self, value: Value) -> bool {
        self.valid_ranges.iter().any(|r| r.contains(&value))
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    values: Vec<Value>,
}

impl Parsable<Ticket> for Ticket {
    fn parse(input: &str) -> IResult<&str, Ticket> {
        map(separated_list0(char(','), Value::parse), |values| Ticket {
            values,
        })(input)
    }
}

#[derive(Debug, PartialEq)]
struct Notes {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Parsable<Notes> for Notes {
    fn parse(input: &str) -> IResult<&str, Notes> {
        let grammar = tuple((
            terminated(
                separated_list0(line_ending, Rule::parse),
                tuple((line_ending, line_ending)),
            ),
            tuple((tag("your ticket:"), line_ending)),
            terminated(Ticket::parse, tuple((line_ending, line_ending))),
            tuple((tag("nearby tickets:"), line_ending)),
            separated_list0(line_ending, Ticket::parse),
            eof,
        ));
        map(grammar, |(rules, _, my_ticket, _, nearby_tickets, _)| {
            Notes {
                rules,
                my_ticket,
                nearby_tickets,
            }
        })(input)
    }
}

impl Notes {
    fn is_definitely_invalid_value(&self, value: Value) -> bool {
        self.rules.iter().all(|rule| !rule.is_valid(value))
    }

    fn ticket_scanning_error_rate(&self) -> Value {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.values.iter())
            .filter(|&&v| self.is_definitely_invalid_value(v))
            .sum()
    }

    fn find_rules_to_fields_map(&self) -> Vec<usize> {
        let mut graph = DirectedGraph::new(2 * self.rules.len() + 2);
        for i in 1..=self.rules.len() {
            graph.add_edge(0, i);
            for j in self.rules.len() + 1..=2 * self.rules.len() {
                graph.add_edge(i, j);
            }
            graph.add_edge(i + self.rules.len(), 2 * self.rules.len() + 1);
        }

        let valid_tickets = self.nearby_tickets.iter().filter(|ticket| {
            !ticket
                .values
                .iter()
                .any(|&v| self.is_definitely_invalid_value(v))
        });
        for ticket in valid_tickets {
            for (i, rule) in self.rules.iter().enumerate() {
                for (j, value) in ticket.values.iter().enumerate() {
                    if !rule.is_valid(*value) {
                        graph.remove_edge(i + 1, j + self.rules.len() + 1);
                    }
                }
            }
        }

        let flow = graph.max_flow(0, 2 * self.rules.len() + 1);
        flow.adjancency
            .iter()
            .skip(1)
            .take(self.rules.len())
            .map(|edges| *edges.iter().next().unwrap() - self.rules.len() - 1)
            .collect()
    }

    fn departures_product(&self) -> Value {
        let rules2fields = self.find_rules_to_fields_map();
        self.rules
            .iter()
            .enumerate()
            .filter(|(_, r)| r.field.starts_with("departure"))
            .map(|(i, _)| self.my_ticket.values[rules2fields[i]])
            .fold(1, |acc, value| acc * value)
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let (_, notes) = Notes::parse(&buf).unwrap();
    println!(
        "Ticket scanning error rate: {}",
        notes.ticket_scanning_error_rate()
    );
    println!("Departures product: {}", notes.departures_product());
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
        class: 1-3 or 5-7\n\
        row: 6-11 or 33-44\n\
        seat: 13-40 or 45-50\n\
        \n\
        your ticket:\n\
        7,1,14\n\
        \n\
        nearby tickets:\n\
        7,3,47\n\
        40,4,50\n\
        55,2,20\n\
        38,6,12";

    fn notes() -> Notes {
        Notes {
            rules: vec![
                Rule {
                    field: String::from("class"),
                    valid_ranges: vec![(1, 3).into(), (5, 7).into()],
                },
                Rule {
                    field: String::from("row"),
                    valid_ranges: vec![(6, 11).into(), (33, 44).into()],
                },
                Rule {
                    field: String::from("seat"),
                    valid_ranges: vec![(13, 40).into(), (45, 50).into()],
                },
            ],
            my_ticket: Ticket {
                values: vec![7, 1, 14],
            },
            nearby_tickets: vec![
                Ticket {
                    values: vec![7, 3, 47],
                },
                Ticket {
                    values: vec![40, 4, 50],
                },
                Ticket {
                    values: vec![55, 2, 20],
                },
                Ticket {
                    values: vec![38, 6, 12],
                },
            ],
        }
    }

    #[test]
    fn test_parse_value() {
        let (_, value) = Value::parse("123").unwrap();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_parse_inclusive_range() {
        let (_, range) = InclusiveRange::parse("12-24").unwrap();
        assert_eq!(range, InclusiveRange(12, 24));
    }

    #[test]
    fn test_parse_rule() {
        let (_, rule) = Rule::parse("label foo: 1-3 or 23-42").unwrap();
        assert_eq!(
            rule,
            Rule {
                field: String::from("label foo"),
                valid_ranges: vec![(1, 3).into(), (23, 42).into()]
            }
        );
    }

    #[test]
    fn test_parsing() {
        let (remainder, parsed_notes) = Notes::parse(INPUT).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(parsed_notes, notes());
    }

    #[test]
    fn test_ticket_scanning_error_rate() {
        assert_eq!(notes().ticket_scanning_error_rate(), 71);
    }
}
