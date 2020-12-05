use itertools::{process_results, Itertools};
use std::convert::{TryFrom, TryInto};
use std::io::{self, BufRead};

enum BinPartitionSelector {
    Lower,
    Upper,
}

fn bin_part(bounds: (usize, usize), selector: BinPartitionSelector) -> (usize, usize) {
    if bounds.0 + 1 >= bounds.1 {
        panic!(
            "Upper bound must be at least two higher than lower bound to \
            allow splitting into binary partitions."
        );
    }

    let mid = bounds.0 + (bounds.1 - bounds.0) / 2;
    match selector {
        BinPartitionSelector::Lower => (bounds.0, mid),
        BinPartitionSelector::Upper => (mid, bounds.1),
    }
}

#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl TryFrom<&[u8; 10]> for Seat {
    type Error = String;

    fn try_from(encoded: &[u8; 10]) -> Result<Self, Self::Error> {
        let row = encoded[..7]
            .iter()
            .map(|c| match c {
                b'F' => Ok(BinPartitionSelector::Lower),
                b'B' => Ok(BinPartitionSelector::Upper),
                _ => Err(format!("Invalid row partition character '{}'.", c)),
            })
            .fold_results((0, 128), bin_part)?
            .0;
        let col = encoded[7..]
            .iter()
            .map(|c| match c {
                b'L' => Ok(BinPartitionSelector::Lower),
                b'R' => Ok(BinPartitionSelector::Upper),
                _ => Err(format!("Invalid column partition character '{}'.", c)),
            })
            .fold_results((0, 8), bin_part)?
            .0;
        Ok(Seat { row, col })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let seats = stdin
        .lock()
        .lines()
        .map(|line| -> Result<usize, Box<dyn std::error::Error>> {
            let encoding: [u8; 10] = line?.as_bytes().try_into()?;
            let seat = Seat::try_from(&encoding)?;
            Ok(seat.seat_id())
        });
    let max_seat_id = process_results(seats, |iter| iter.max())?.ok_or("No seat IDs.")?;
    println!("Max seat id: {}", max_seat_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(bounds, selector, expected,
        case((0, 64), BinPartitionSelector::Lower, (0, 32)),
        case((0, 64), BinPartitionSelector::Upper, (32, 64)),
        case((64, 128), BinPartitionSelector::Lower, (64, 96)),
        case((64, 128), BinPartitionSelector::Upper, (96, 128)),
        case((2, 9), BinPartitionSelector::Lower, (2, 5)),
        case((2, 9), BinPartitionSelector::Upper, (5, 9)),
        case((3, 5), BinPartitionSelector::Lower, (3, 4)),
        case((3, 5), BinPartitionSelector::Upper, (4, 5))
    )]
    fn bin_part_selects_correct_range(
        bounds: (usize, usize),
        selector: BinPartitionSelector,
        expected: (usize, usize),
    ) {
        assert_eq!(bin_part(bounds, selector), expected);
    }

    #[rstest(bounds => [(4, 5), (5, 5), (5, 4)])]
    #[should_panic]
    fn bin_part_panics_with_invalid_input_bounds(bounds: (usize, usize)) {
        bin_part(bounds, BinPartitionSelector::Lower);
    }

    #[rstest(seat, expected_id,
        case(Seat { row: 44, col: 5 }, 357),
        case(Seat { row: 70, col: 7 }, 567),
        case(Seat { row: 14, col: 7 }, 119),
        case(Seat { row: 102, col: 4 }, 820),
    )]
    fn test_seat_id(seat: Seat, expected_id: usize) {
        assert_eq!(seat.seat_id(), expected_id);
    }

    #[rstest(encoding, expected_seat,
        case(b"FBFBBFFRLR", Seat { row: 44, col: 5 }),
        case(b"BFFFBBFRRR", Seat { row: 70, col: 7 }),
        case(b"FFFBBBFRRR", Seat { row: 14, col: 7 }),
        case(b"BBFFBBFRLL", Seat { row: 102, col: 4 }),
    )]
    fn try_to_obtain_seat_from_binary_space_partitioning_encoding(
        encoding: &[u8; 10],
        expected_seat: Seat,
    ) {
        let seat = Seat::try_from(encoding).unwrap();
        assert_eq!(seat, expected_seat);
    }
}
