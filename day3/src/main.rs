use std::io::{self, BufRead};

fn main() {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let trees_hit = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .fold(vec![0; slopes.len()], |trees_hit, (i, line)| {
            slopes
                .iter()
                .zip(trees_hit)
                .map(|((slope_right, slope_down), trees_hit_on_slope)| {
                    trees_hit_on_slope
                        + if i % slope_down == 0 {
                            let x = (i as f64 / *slope_down as f64).ceil() as usize;
                            match line.as_bytes()[(slope_right * x) % line.len()] {
                                b'#' => 1,
                                b'.' => 0,
                                c => panic!("Invalid input {}", c),
                            }
                        } else {
                            0
                        }
                })
                .collect()
        });
    println!(
        "Trees hit on slope (3, 1): {}",
        trees_hit[slopes.iter().position(|x| *x == (3, 1)).unwrap()]
    );
    println!(
        "Product of trees hit on all slopes: {}",
        trees_hit.iter().fold(1, |x: u64, y| x * y)
    );
}
