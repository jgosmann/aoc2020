use std::io::{self, BufRead};

fn main() {
    println!(
        "Trees hit: {}",
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .enumerate()
            .fold(0, |trees_hit: usize, (i, line)| {
                trees_hit
                    + match line.as_bytes()[(3 * i) % line.len()] {
                        b'#' => 1,
                        b'.' => 0,
                        c => panic!("Invalid input {}", c),
                    }
            })
    );
}
