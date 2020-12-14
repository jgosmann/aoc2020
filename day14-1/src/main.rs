use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::num::ParseIntError;

type Address = u64;
type Value = u64;

#[derive(Debug, PartialEq)]
struct Mask {
    zero_mask: Value,
    one_mask: Value,
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            zero_mask: u64::MAX,
            one_mask: 0,
        }
    }
}

impl Mask {
    fn apply(&self, value: Value) -> Value {
        (value & self.zero_mask) | self.one_mask
    }
}

impl TryFrom<&str> for Mask {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let zero_mask = value.replace('X', "1");
        let zero_mask = Value::from_str_radix(&zero_mask, 2)?;
        let one_mask = value.replace('X', "0");
        let one_mask = Value::from_str_radix(&one_mask, 2)?;
        Ok(Self {
            zero_mask,
            one_mask,
        })
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    SetMask(Mask),
    SetMem(Address, Value),
}

enum OpCodeParseError {
    InvalidStatement,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for OpCodeParseError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl OpCode {
    pub fn parse_statement(input: &str) -> Result<OpCode, String> {
        use nom::{
            character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
            combinator::{eof, map_res, opt},
            sequence::{delimited, tuple},
        };
        let address = delimited(char('['), digit1, char(']'));
        let assignment = delimited(multispace0, char('='), multispace0);
        let grammar = tuple((alpha1, opt(address), assignment, alphanumeric1, eof));
        let mut parser = map_res(grammar, |(keyword, address, _, value, _)| {
            match (keyword, address) {
                ("mask", None) => Ok(OpCode::SetMask(Mask::try_from(value)?)),
                ("mem", Some(address)) => Ok(OpCode::SetMem(address.parse()?, value.parse()?)),
                _ => Err(OpCodeParseError::InvalidStatement),
            }
        });
        parser(input)
            .map(|(_, op_code)| op_code)
            .map_err(|err: nom::Err<(&str, _)>| format!("{}", err))
    }
}

struct ComputerSystem {
    current_mask: Mask,
    mem: HashMap<u64, u64>,
}

impl ComputerSystem {
    fn new() -> Self {
        Self {
            current_mask: Mask::default(),
            mem: HashMap::new(),
        }
    }

    fn execute(&mut self, operation: OpCode) {
        match operation {
            OpCode::SetMask(mask) => self.current_mask = mask,
            OpCode::SetMem(address, value) => {
                self.mem.insert(address, self.current_mask.apply(value));
            }
        }
    }
}

fn run_program(program: impl Iterator<Item = impl AsRef<str>>) -> Result<Value, String> {
    let mut computer = ComputerSystem::new();
    for statement in program {
        let op_code = OpCode::parse_statement(statement.as_ref())?;
        computer.execute(op_code);
    }
    Ok(computer.mem.values().sum())
}

fn main() {
    let stdin = io::stdin();
    println!(
        "{}",
        run_program(stdin.lock().lines().map(Result::unwrap)).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROGRAM: [&str; 4] = [
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ];

    #[test]
    fn test_opcode_parse_set_mask_statement() {
        let opcode =
            OpCode::parse_statement("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
        assert_eq!(
            opcode,
            OpCode::SetMask(Mask {
                one_mask: 0,
                zero_mask: 0x0fffffffff
            })
        );
    }

    #[test]
    fn test_opcode_parse_set_mem_statement() {
        let opcode = OpCode::parse_statement("mem[42] = 23").unwrap();
        assert_eq!(opcode, OpCode::SetMem(42, 23));
    }

    #[test]
    fn test_program() {
        assert_eq!(run_program(PROGRAM.iter()).unwrap(), 165);
    }
}
