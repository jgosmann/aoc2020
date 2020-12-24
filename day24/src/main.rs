use std::collections::HashSet;
use std::io::{self, BufRead};

type Index = (isize, isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HexNeighbour {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexNeighbour {
    fn from_char_iter<'a>(
        input: &'a mut impl Iterator<Item = char>,
    ) -> HexNeighbourParser<'a, impl Iterator<Item = char>> {
        HexNeighbourParser { input }
    }

    fn of_index(&self, index: Index) -> Index {
        use HexNeighbour::*;
        match self {
            East => (index.0 + 1, index.1),
            SouthEast => (index.0 + 1, index.1 - 1),
            SouthWest => (index.0, index.1 - 1),
            West => (index.0 - 1, index.1),
            NorthWest => (index.0 - 1, index.1 + 1),
            NorthEast => (index.0, index.1 + 1),
        }
    }

    fn get_index(path: &mut impl Iterator<Item = Self>) -> Index {
        path.fold((0, 0), |index, neighbour| neighbour.of_index(index))
    }

    fn all() -> [Self; 6] {
        use HexNeighbour::*;
        [East, SouthEast, SouthWest, West, NorthWest, NorthEast]
    }
}

fn get_flipped_tiles(input: impl Iterator<Item = impl AsRef<str>>) -> HashSet<Index> {
    let mut flipped = HashSet::new();
    for line in input {
        let index = HexNeighbour::get_index(&mut HexNeighbour::from_char_iter(
            &mut line.as_ref().chars(),
        ));
        if flipped.contains(&index) {
            flipped.remove(&index);
        } else {
            flipped.insert(index);
        }
    }
    flipped
}

fn neighbours_of(index: Index) -> Vec<Index> {
    HexNeighbour::all()
        .iter()
        .map(|n| n.of_index(index))
        .collect()
}

fn advance_day(flipped_state: HashSet<Index>) -> HashSet<Index> {
    let mut new_flipped_state = HashSet::new();
    let white_tiles_to_consider: Vec<Index> = flipped_state
        .iter()
        .flat_map(|&black_tile| {
            neighbours_of(black_tile)
                .iter()
                .filter(|index| !flipped_state.contains(index))
                .copied()
                .collect::<Vec<Index>>()
        })
        .collect();

    for black_tile in &flipped_state {
        let n_black_neighbours = neighbours_of(*black_tile)
            .iter()
            .filter(|tile| flipped_state.contains(tile))
            .count();
        if 0 < n_black_neighbours && n_black_neighbours <= 2 {
            new_flipped_state.insert(*black_tile);
        }
    }

    for white_tile in &white_tiles_to_consider {
        let n_black_neighbours = neighbours_of(*white_tile)
            .iter()
            .filter(|tile| flipped_state.contains(tile))
            .count();
        if n_black_neighbours == 2 {
            new_flipped_state.insert(*white_tile);
        }
    }

    new_flipped_state
}

fn advance_n_days(flipped_state: HashSet<Index>, n_days: usize) -> HashSet<Index> {
    let mut flipped_state = flipped_state;
    for _ in 0..n_days {
        flipped_state = advance_day(flipped_state);
    }
    flipped_state
}

struct HexNeighbourParser<'a, I>
where
    I: Iterator<Item = char>,
{
    input: &'a mut I,
}

impl<'a, I> Iterator for HexNeighbourParser<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = HexNeighbour;

    fn next(&mut self) -> Option<Self::Item> {
        use HexNeighbour::*;
        match self.input.next() {
            Some('e') => Some(East),
            Some('w') => Some(West),
            Some('s') => match self.input.next() {
                Some('e') => Some(SouthEast),
                Some('w') => Some(SouthWest),
                _ => None,
            },
            Some('n') => match self.input.next() {
                Some('e') => Some(NorthEast),
                Some('w') => Some(NorthWest),
                _ => None,
            },
            _ => None,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let flipped = get_flipped_tiles(&mut lines);
    println!("Black tiles initially: {}", flipped.len());
    println!(
        "Black tiles after 100 days: {}",
        advance_n_days(flipped, 100).len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
    }

    #[test]
    fn test_hex_neighbour_from_char_iter() {
        use HexNeighbour::*;
        assert_eq!(
            HexNeighbour::from_char_iter(&mut "sesenwnee".chars()).collect::<Vec<HexNeighbour>>(),
            vec![SouthEast, SouthEast, NorthWest, NorthEast, East]
        );
    }

    #[test]
    fn test_get_flipped_tiles() {
        assert_eq!(get_flipped_tiles(&mut input().iter()).len(), 10);
    }

    #[test]
    fn test_advance_day() {
        assert_eq!(
            advance_day(get_flipped_tiles(&mut input().iter())).len(),
            15
        );
    }
    #[test]

    fn test_advance_n_days() {
        assert_eq!(
            advance_n_days(get_flipped_tiles(&mut input().iter()), 100).len(),
            2208
        );
    }
}
