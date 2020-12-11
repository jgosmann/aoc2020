use std::convert::TryFrom;
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
                (GridPos::Seat(true), n_occupied_neighbours) if n_occupied_neighbours >= 4 => {
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
        let (x, y) = pos;
        let mut neighbours = Vec::with_capacity(8);
        if x > 0 {
            if y > 0 {
                neighbours.push((x - 1, y - 1));
            }
            neighbours.push((x - 1, y));
            if y < self.n_columns - 1 {
                neighbours.push((x - 1, y + 1))
            }
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.n_columns - 1 {
            neighbours.push((x, y + 1));
        }
        if x < self.n_rows - 1 {
            if y > 0 {
                neighbours.push((x + 1, y - 1));
            }
            neighbours.push((x + 1, y));
            if y < self.n_columns - 1 {
                neighbours.push((x + 1, y + 1))
            }
        }
        neighbours
    }

    fn pos2idx(&self, pos: (usize, usize)) -> usize {
        pos.0 * self.n_columns + pos.1
    }

    fn idx2pos(&self, idx: usize) -> (usize, usize) {
        (idx / self.n_columns, idx % self.n_columns)
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
                "#.##.L#.##",
                "#L###LL.L#",
                "L.#.#..#..",
                "#L##.##.L#",
                "#.##.LL.LL",
                "#.###L#.##",
                "..#.#.....",
                "#L######L#",
                "#.LL###L.L",
                "#.#L###.##",
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
                "#.#L.L#.##",
                "#LLL#LL.L#",
                "L.#.L..#..",
                "#L##.##.L#",
                "#.#L.LL.LL",
                "#.#L#L#.##",
                "..L.L.....",
                "#L#L##L#L#",
                "#.LLLLLL.L",
                "#.#L#L#.##",
            ]
            .iter(),
        )
        .unwrap();
        input.advance_to_stable_state();
        assert_eq!(input.state, expected.state);
        assert_eq!(input.iter_seats().filter(|s| s.is_occupied()).count(), 37);
    }
}
