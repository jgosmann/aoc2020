use std::cmp::{max, min};
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::io::{self, Read};
use std::num::TryFromIntError;

type Idx3 = (i64, i64, i64, i64);

#[derive(Debug, PartialEq)]
struct ConwayCube {
    active: HashSet<Idx3>,
    lower_bounds: Idx3,
    upper_bounds: Idx3,
}

#[derive(Debug, PartialEq)]
enum ConwayCubeParseError {
    InvalidCharacter(char),
    OutOfRange(TryFromIntError),
}

impl TryFrom<&str> for ConwayCube {
    type Error = ConwayCubeParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let active: HashSet<Idx3> = input
            .split_ascii_whitespace()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars().enumerate().filter_map(move |(col_idx, c)| {
                    let row_idx = i64::try_from(row_idx);
                    let col_idx = i64::try_from(col_idx);
                    match (c, row_idx, col_idx) {
                        ('#', Ok(row_idx), Ok(col_idx)) => Some(Ok((row_idx, col_idx, 0, 0))),
                        ('.', Ok(_), Ok(_)) => None,
                        (_, Err(row_err), _) => {
                            Some(Err(ConwayCubeParseError::OutOfRange(row_err)))
                        }
                        (_, _, Err(col_err)) => {
                            Some(Err(ConwayCubeParseError::OutOfRange(col_err)))
                        }
                        _ => Some(Err(ConwayCubeParseError::InvalidCharacter(c))),
                    }
                })
            })
            .collect::<Result<HashSet<Idx3>, ConwayCubeParseError>>()?;
        let bounds = active
            .iter()
            .fold(((0, 0, 0, 0), (0, 0, 0, 0)), Self::extend_bounds);
        Ok(Self {
            active,
            lower_bounds: bounds.0,
            upper_bounds: bounds.1,
        })
    }
}

impl ConwayCube {
    fn extend_bounds(bounds: (Idx3, Idx3), p: &Idx3) -> (Idx3, Idx3) {
        let (lb, ub) = bounds;
        (
            (
                min(lb.0, p.0),
                min(lb.1, p.1),
                min(lb.2, p.2),
                min(lb.3, p.3),
            ),
            (
                max(ub.0, p.0),
                max(ub.1, p.1),
                max(ub.2, p.2),
                max(ub.3, p.3),
            ),
        )
    }

    fn neighbours(p: &Idx3) -> [Idx3; 80] {
        (-1..=1)
            .flat_map(|x| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).flat_map(move |z| {
                        (-1..=1).map(move |zz| (p.0 + x, p.1 + y, p.2 + z, p.3 + zz))
                    })
                })
            })
            .filter(|neighbour| neighbour != p)
            .collect::<Vec<Idx3>>()
            .try_into()
            .unwrap()
    }

    fn new(capacity: usize) -> Self {
        Self {
            active: HashSet::with_capacity(capacity),
            lower_bounds: (0, 0, 0, 0),
            upper_bounds: (0, 0, 0, 0),
        }
    }

    fn activate(&mut self, p: &Idx3) {
        self.active.insert(*p);
        let bounds = Self::extend_bounds((self.lower_bounds, self.upper_bounds), p);
        self.lower_bounds = bounds.0;
        self.upper_bounds = bounds.1;
    }

    fn next_state(self) -> Self {
        let mut state = ConwayCube::new(self.active.len());
        for x in self.lower_bounds.0 - 1..=self.upper_bounds.0 + 1 {
            for y in self.lower_bounds.1 - 1..=self.upper_bounds.1 + 1 {
                for z in self.lower_bounds.2 - 1..=self.upper_bounds.2 + 1 {
                    for zz in self.lower_bounds.3 - 1..=self.upper_bounds.3 + 1 {
                        let p = (x, y, z, zz);
                        let n_neighbours_active = Self::neighbours(&p)
                            .iter()
                            .filter(|p| self.active.contains(p))
                            .count();
                        if self.active.contains(&p)
                            && n_neighbours_active >= 2
                            && n_neighbours_active <= 3
                        {
                            state.activate(&p);
                        } else if !self.active.contains(&p) && n_neighbours_active == 3 {
                            state.activate(&p);
                        }
                    }
                }
            }
        }
        state
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let cube = ConwayCube::try_from(buf.as_str()).unwrap();
    let cube = (0..6).fold(cube, |cube, _| cube.next_state());
    println!("{}", cube.active.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    static INPUT: &str = "\
            .#.\n\
            ..#\n\
            ###\n";

    #[test]
    fn test_parsing() {
        let cube = ConwayCube::try_from(INPUT).unwrap();
        assert_eq!(
            cube,
            ConwayCube {
                active: HashSet::from_iter(
                    [
                        (0, 1, 0, 0),
                        (1, 2, 0, 0),
                        (2, 0, 0, 0),
                        (2, 1, 0, 0),
                        (2, 2, 0, 0)
                    ]
                    .iter()
                    .copied()
                ),
                lower_bounds: (0, 0, 0, 0),
                upper_bounds: (2, 2, 0, 0),
            }
        )
    }

    #[test]
    fn test_conway_cube() {
        let cube = ConwayCube::try_from(INPUT).unwrap();
        let cube = (0..6).fold(cube, |cube, _| cube.next_state());
        assert_eq!(cube.active.len(), 848);
    }
}
