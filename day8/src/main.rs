use std::fmt::Display;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum OpCode {
    Acc(i32),
    Jmp(isize),
    Nop,
}

#[derive(Debug, PartialEq)]
struct State {
    accumulator: i32,
    instruction_pointer: usize,
}

impl State {
    fn new() -> Self {
        Self {
            accumulator: 0,
            instruction_pointer: 0,
        }
    }
}

#[derive(Debug)]
enum OpCodeParseError {
    InvalidOpCode,
}

impl Display for OpCodeParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            OpCodeParseError::InvalidOpCode => "Invalid op code.",
        })
    }
}

impl std::error::Error for OpCodeParseError {}

impl OpCode {
    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let tokens: Vec<&str> = input.split(' ').collect();
        match tokens[0] {
            "acc" => Ok(OpCode::Acc(tokens[1].parse()?)),
            "jmp" => Ok(OpCode::Jmp(tokens[1].parse()?)),
            "nop" => Ok(OpCode::Nop),
            _ => Err(Box::new(OpCodeParseError::InvalidOpCode)),
        }
    }
}

fn reduce(state: State, operation: &OpCode) -> State {
    match operation {
        OpCode::Acc(value) => State {
            accumulator: state.accumulator + value,
            instruction_pointer: state.instruction_pointer + 1,
        },
        OpCode::Jmp(value) => State {
            accumulator: state.accumulator,
            instruction_pointer: if value.is_negative() {
                state.instruction_pointer - value.wrapping_abs() as usize
            } else {
                state.instruction_pointer + *value as usize
            },
        },
        OpCode::Nop => State {
            accumulator: state.accumulator,
            instruction_pointer: state.instruction_pointer + 1,
        },
    }
}

fn detect_loop(program: Vec<OpCode>) -> State {
    let mut instructions_hit = vec![false; program.len()];
    let mut state = State::new();
    loop {
        if instructions_hit[state.instruction_pointer] {
            return state;
        }

        instructions_hit[state.instruction_pointer] = true;
        let operation = &program[state.instruction_pointer];
        state = reduce(state, operation);
    }
}

fn main() {
    let program: Vec<OpCode> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| OpCode::parse(&line).unwrap())
        .collect();
    let loop_state = detect_loop(program);
    println!("{:?}", loop_state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, expected,
        case("acc +1", OpCode::Acc(1)),
        case("acc -2", OpCode::Acc(-2)),
        case("jmp +3", OpCode::Jmp(3)),
        case("jmp -4", OpCode::Jmp(-4)),
        case("nop +5", OpCode::Nop),
        case("nop -6", OpCode::Nop),
    )]
    fn test_opcode_parsing(input: &str, expected: OpCode) {
        assert_eq!(OpCode::parse(input).unwrap(), expected);
    }

    #[rstest(state, operation, new_state,
        case(State::new(), OpCode::Acc(10), State { accumulator: 10, instruction_pointer: 1 }),
        case(State::new(), OpCode::Acc(-10), State { accumulator: -10, instruction_pointer: 1 }),
        case(State::new(), OpCode::Jmp(10), State { accumulator: 0, instruction_pointer: 10 }),
        case(State { accumulator: 0, instruction_pointer: 20 }, OpCode::Jmp(-10), State { accumulator: 0, instruction_pointer: 10 }),
        case(State::new(), OpCode::Nop, State { accumulator: 0, instruction_pointer: 1 }),
    )]
    fn test_reducer(state: State, operation: OpCode, new_state: State) {
        assert_eq!(reduce(state, &operation), new_state);
    }

    #[test]
    fn test_detect_loop() {
        let program = vec![
            OpCode::Nop,
            OpCode::Acc(1),
            OpCode::Jmp(4),
            OpCode::Acc(3),
            OpCode::Jmp(-3),
            OpCode::Acc(-99),
            OpCode::Acc(1),
            OpCode::Jmp(-4),
            OpCode::Acc(6),
        ];
        assert_eq!(
            detect_loop(program),
            State {
                accumulator: 5,
                instruction_pointer: 1
            }
        );
    }
}
