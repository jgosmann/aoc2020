use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Write};
use std::io::{self, BufRead};
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
enum GridPos {
    Floor,
    Seat(bool),
}

impl GridPos {
    fn is_occupied(&self) -> bool {
        match self {
            Self::Floor => false,
            Self::Seat(is_occupied) => *is_occupied,
        }
    }
}

impl TryFrom<char> for GridPos {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, &'static str> {
        match c {
            '.' => Ok(GridPos::Floor),
            'L' => Ok(GridPos::Seat(false)),
            '#' => Ok(GridPos::Seat(true)),
            _ => Err("Invalid seat marker."),
        }
    }
}

#[derive(Debug, PartialEq)]
struct FerryCellularAutomaton {
    state: Vec<GridPos>,
    state_buffer: Vec<GridPos>,
    n_columns: usize,
    n_rows: usize,
}

impl FerryCellularAutomaton {
    pub fn parse(lines_iter: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, &'static str> {
        let grid = lines_iter
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(GridPos::try_from)
                    .collect::<Result<Vec<GridPos>, &'static str>>()
            })
            .collect::<Result<Vec<Vec<GridPos>>, &'static str>>()?;
        let n_columns = grid[0].len();
        if !grid.iter().all(|row| row.len() == n_columns) {
            return Err("All rows must have the same number of columns.");
        }
        let n_rows = grid.len();
        let total_len = n_columns * n_rows;
        Ok(Self {
            state: grid.into_iter().flatten().collect(),
            state_buffer: vec![GridPos::Floor; total_len],
            n_columns,
            n_rows,
        })
    }

    pub fn advance(&mut self) {
        for (i, seat) in self.state.iter().enumerate() {
            let n_occupied_neighbours = self
                .neighbours(self.idx2pos(i))
                .iter()
                .filter(|&&neighbour| self.state[self.pos2idx(neighbour)].is_occupied())
                .count();
            self.state_buffer[i] = match (seat, n_occupied_neighbours) {
                (GridPos::Seat(false), 0) => GridPos::Seat(true),
                (GridPos::Seat(true), n_occupied_neighbours) if n_occupied_neighbours >= 5 => {
                    GridPos::Seat(false)
                }
                (seat, _) => *seat,
            };
        }
        mem::swap(&mut self.state, &mut self.state_buffer);
    }

    pub fn advance_to_stable_state(&mut self) {
        while self.state != self.state_buffer {
            self.advance();
        }
    }

    pub fn iter_seats(&self) -> impl Iterator<Item = &GridPos> {
        self.state.iter()
    }

    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(8);
        for xd in -1..=1 {
            for yd in -1..=1 {
                if xd != 0 || yd != 0 {
                    if let Some(seat_pos) = self.find_seat_in_direction(pos, (xd, yd)) {
                        neighbours.push(seat_pos);
                    }
                }
            }
        }
        neighbours
    }

    fn find_seat_in_direction(
        &self,
        starting_pos: (usize, usize),
        move_vec: (isize, isize),
    ) -> Option<(usize, usize)> {
        let starting_pos = (starting_pos.0 as isize, starting_pos.1 as isize);
        let mut current_pos = (starting_pos.0 + move_vec.0, starting_pos.1 + move_vec.1);
        while 0 <= current_pos.0
            && (current_pos.0 as usize) < self.n_rows
            && 0 <= current_pos.1
            && (current_pos.1 as usize) < self.n_columns
        {
            let pos = (current_pos.0 as usize, current_pos.1 as usize);
            if self.state[self.pos2idx(pos)] != GridPos::Floor {
                return Some(pos);
            }
            current_pos = (current_pos.0 + move_vec.0, current_pos.1 + move_vec.1);
        }
        None
    }

    fn pos2idx(&self, pos: (usize, usize)) -> usize {
        pos.0 * self.n_columns + pos.1
    }

    fn idx2pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.n_columns, idx % self.n_columns)
    }
}

impl Display for FerryCellularAutomaton {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        for (i, seat) in self.iter_seats().enumerate() {
            let c = match seat {
                GridPos::Floor => '.',
                GridPos::Seat(false) => 'L',
                GridPos::Seat(true) => '#',
            };
            f.write_char(c)?;
            if (i + 1) % self.n_columns == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let lines_iter = stdin.lock().lines().map(Result::unwrap);
    let mut automaton = FerryCellularAutomaton::parse(lines_iter).unwrap();
    automaton.advance_to_stable_state();
    println!(
        "Occupied seats: {}",
        automaton.iter_seats().filter(|s| s.is_occupied()).count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static STARTING_STATE: [&str; 10] = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    #[test]
    fn test_parsing() {
        let input = ["L#", ".L"];
        let automaton = FerryCellularAutomaton::parse(input.iter());
        assert_eq!(
            automaton,
            Ok(FerryCellularAutomaton {
                state: vec![
                    GridPos::Seat(false),
                    GridPos::Seat(true),
                    GridPos::Floor,
                    GridPos::Seat(false),
                ],
                state_buffer: vec![
                    GridPos::Floor,
                    GridPos::Floor,
                    GridPos::Floor,
                    GridPos::Floor,
                ],
                n_columns: 2,
                n_rows: 2,
            })
        );
    }

    #[test]
    fn test_advancing() {
        let mut input = FerryCellularAutomaton::parse(STARTING_STATE.iter()).unwrap();
        let expected = FerryCellularAutomaton::parse(
            [
                "#.L#.##.L#",
                "#L#####.LL",
                "L.#.#..#..",
                "##L#.##.##",
                "#.##.#L.##",
                "#.#####.#L",
                "..#.#.....",
                "LLL####LL#",
                "#.L#####.L",
                "#.L####.L#",
            ]
            .iter(),
        )
        .unwrap();
        for _ in 0..3 {
            input.advance();
        }
        assert_eq!(input.state, expected.state);
    }

    #[test]
    fn test_advance_to_stable_state() {
        let mut input = FerryCellularAutomaton::parse(STARTING_STATE.iter()).unwrap();
        let expected = FerryCellularAutomaton::parse(
            [
                "#.L#.L#.L#",
                "#LLLLLL.LL",
                "L.L.L..#..",
                "##L#.#L.L#",
                "L.L#.LL.L#",
                "#.LLLL#.LL",
                "..#.L.....",
                "LLL###LLL#",
                "#.LLLLL#.L",
                "#.L#LL#.L#",
            ]
            .iter(),
        )
        .unwrap();
        input.advance_to_stable_state();
        assert_eq!(input.state, expected.state);
        assert_eq!(input.iter_seats().filter(|s| s.is_occupied()).count(), 26);
    }
}
