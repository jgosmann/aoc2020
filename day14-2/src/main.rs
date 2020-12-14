use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::num::ParseIntError;

type Address = u64;
type Value = u64;

#[derive(Debug, PartialEq)]
struct Decoder {
    one_mask: Address,
    fluctuating_bits: Vec<usize>,
}

impl Default for Decoder {
    fn default() -> Self {
        Decoder {
            fluctuating_bits: vec![],
            one_mask: 0,
        }
    }
}

struct AddressIterator<'a> {
    base_address: Address,
    fluctuating_bits: &'a Vec<usize>,
    fluctuating_mask: Address,
    state: usize,
}

impl<'a> AddressIterator<'a> {
    fn new(address: Address, decoder: &'a Decoder) -> Self {
        Self {
            base_address: address | decoder.one_mask,
            fluctuating_bits: &decoder.fluctuating_bits,
            fluctuating_mask: Self::bits2mask(&decoder.fluctuating_bits, usize::MAX),
            state: 0,
        }
    }

    fn bits2mask(bits: &Vec<usize>, bit_selector: usize) -> Address {
        bits.iter().enumerate().fold(0, |mask, (i, bit)| {
            if bit_selector & (1 << i) != 0 {
                mask | (1 << bit)
            } else {
                mask
            }
        })
    }
}

impl<'a> Iterator for AddressIterator<'a> {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state < (1 << self.fluctuating_bits.len()) {
            let mask = Self::bits2mask(self.fluctuating_bits, self.state);
            self.state += 1;
            Some((self.base_address & !self.fluctuating_mask) | mask)
        } else {
            None
        }
    }
}

impl Decoder {
    fn iter_addresses<'a>(&'a self, base_address: Address) -> AddressIterator<'a> {
        AddressIterator::new(base_address, self)
    }
}

impl TryFrom<&str> for Decoder {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let one_mask = value.replace('X', "0");
        let one_mask = Value::from_str_radix(&one_mask, 2)?;
        Ok(Self {
            one_mask,
            fluctuating_bits: value
                .as_bytes()
                .iter()
                .rev()
                .enumerate()
                .filter_map(|(i, &c)| if c == b'X' { Some(i) } else { None })
                .collect(),
        })
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    SetDecoder(Decoder),
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
                ("mask", None) => Ok(OpCode::SetDecoder(Decoder::try_from(value)?)),
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
    current_decoder: Decoder,
    mem: HashMap<u64, u64>,
}

impl ComputerSystem {
    fn new() -> Self {
        Self {
            current_decoder: Decoder::default(),
            mem: HashMap::new(),
        }
    }

    fn execute(&mut self, operation: OpCode) {
        match operation {
            OpCode::SetDecoder(decoder) => self.current_decoder = decoder,
            OpCode::SetMem(base_address, value) => {
                for address in self.current_decoder.iter_addresses(base_address) {
                    self.mem.insert(address, value);
                }
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
        "mask = 000000000000000000000000000000X1001X",
        "mem[42] = 100",
        "mask = 00000000000000000000000000000000X0XX",
        "mem[26] = 1",
    ];

    #[test]
    fn test_opcode_parse_set_mask_statement() {
        let opcode =
            OpCode::parse_statement("mask = 000000000000000000000000000000X0XX11").unwrap();
        assert_eq!(
            opcode,
            OpCode::SetDecoder(Decoder {
                one_mask: 0b11,
                fluctuating_bits: vec![2, 3, 5],
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
        assert_eq!(run_program(PROGRAM.iter()).unwrap(), 208);
    }
}
