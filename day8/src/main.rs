use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq)]
enum OpCode {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

#[derive(Clone, Debug, PartialEq)]
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
            "nop" => Ok(OpCode::Nop(tokens[1].parse()?)),
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
        OpCode::Nop(_) => State {
            accumulator: state.accumulator,
            instruction_pointer: state.instruction_pointer + 1,
        },
    }
}

fn construct_reverse_flow_graph(program: &[OpCode]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; program.len() + 1];

    for (i, operation) in program.iter().enumerate() {
        let new_state = reduce(
            State {
                accumulator: 0,
                instruction_pointer: i,
            },
            operation,
        );
        graph[new_state.instruction_pointer].push(i)
    }

    graph
}

fn determine_halting_nodes(reverse_flow_graph: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut halting_nodes = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(reverse_flow_graph.len() - 1);

    while let Some(node_idx) = queue.pop_front() {
        halting_nodes.insert(node_idx.clone());
        for preceding_node in &reverse_flow_graph[node_idx] {
            queue.push_back(preceding_node.clone());
        }
    }

    halting_nodes
}

fn detect_loop(program: &[OpCode]) -> State {
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

fn execute_program_with_self_healing(program: &[OpCode]) -> i32 {
    let mut state = State::new();
    let halting_nodes = determine_halting_nodes(&construct_reverse_flow_graph(program));
    let mut fixed = false;

    while state.instruction_pointer < program.len() {
        let operation = &program[state.instruction_pointer];

        let flipped = match (fixed, operation) {
            (false, OpCode::Jmp(value)) => Some(OpCode::Nop(*value)),
            (false, OpCode::Nop(value)) => Some(OpCode::Jmp(*value)),
            _ => None,
        };
        if let Some(flipped) = flipped {
            let state_with_flipping = reduce(state.clone(), &flipped);
            if halting_nodes.contains(&state_with_flipping.instruction_pointer) {
                state = state_with_flipping;
                fixed = true;
                continue;
            }
        }

        state = reduce(state, operation);
    }

    state.accumulator
}

fn main() {
    let program: Vec<OpCode> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| OpCode::parse(&line).unwrap())
        .collect();

    let loop_state = detect_loop(&program);
    println!("loop_state: {:?}", loop_state);

    let result = execute_program_with_self_healing(&program);
    println!("result of fixed program: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static PROGRAM: [OpCode; 9] = [
        OpCode::Nop(0),
        OpCode::Acc(1),
        OpCode::Jmp(4),
        OpCode::Acc(3),
        OpCode::Jmp(-3),
        OpCode::Acc(-99),
        OpCode::Acc(1),
        OpCode::Jmp(-4),
        OpCode::Acc(6),
    ];

    #[rstest(input, expected,
        case("acc +1", OpCode::Acc(1)),
        case("acc -2", OpCode::Acc(-2)),
        case("jmp +3", OpCode::Jmp(3)),
        case("jmp -4", OpCode::Jmp(-4)),
        case("nop +5", OpCode::Nop(5)),
        case("nop -6", OpCode::Nop(-6)),
    )]
    fn test_opcode_parsing(input: &str, expected: OpCode) {
        assert_eq!(OpCode::parse(input).unwrap(), expected);
    }

    #[rstest(state, operation, new_state,
        case(State::new(), OpCode::Acc(10), State { accumulator: 10, instruction_pointer: 1 }),
        case(State::new(), OpCode::Acc(-10), State { accumulator: -10, instruction_pointer: 1 }),
        case(State::new(), OpCode::Jmp(10), State { accumulator: 0, instruction_pointer: 10 }),
        case(State { accumulator: 0, instruction_pointer: 20 }, OpCode::Jmp(-10), State { accumulator: 0, instruction_pointer: 10 }),
        case(State::new(), OpCode::Nop(10), State { accumulator: 0, instruction_pointer: 1 }),
    )]
    fn test_reducer(state: State, operation: OpCode, new_state: State) {
        assert_eq!(reduce(state, &operation), new_state);
    }

    #[test]
    fn test_detect_loop() {
        assert_eq!(
            detect_loop(&PROGRAM),
            State {
                accumulator: 5,
                instruction_pointer: 1
            }
        );
    }

    #[test]
    fn test_construct_reverse_flow_graph() {
        assert_eq!(
            construct_reverse_flow_graph(&PROGRAM),
            vec![
                vec![],
                vec![0, 4],
                vec![1],
                vec![7],
                vec![3],
                vec![],
                vec![2, 5],
                vec![6],
                vec![],
                vec![8],
            ]
        );
    }

    #[test]
    fn test_determine_halting_nodes() {
        assert_eq!(
            determine_halting_nodes(&construct_reverse_flow_graph(&PROGRAM)),
            [8, 9].iter().cloned().collect()
        );
    }

    #[test]
    fn test_execute_program_with_self_healing() {
        assert_eq!(execute_program_with_self_healing(&PROGRAM), 8);
    }
}
