use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

struct XmasProcessor {
    preamble_len: usize,
    window_queue: VecDeque<u64>,
    window_tree: HashSet<u64>,
}

impl XmasProcessor {
    fn new(preamble_len: usize) -> Self {
        XmasProcessor {
            preamble_len,
            window_queue: VecDeque::with_capacity(preamble_len),
            window_tree: HashSet::new(),
        }
    }

    fn push(&mut self, value: u64) -> bool {
        let result = if self.preamble_len <= self.window_queue.len() {
            let result = self.window_queue.iter().any(|x| {
                if let Some(diff) = value.checked_sub(*x) {
                    self.window_tree.contains(&diff)
                } else {
                    false
                }
            });
            self.window_tree
                .remove(&self.window_queue.pop_front().unwrap());
            result
        } else {
            true
        };
        self.window_queue.push_back(value);
        self.window_tree.insert(value);
        result
    }
}

fn find_contiguous_range_with_sum(values: &[u64], target_sum: u64) -> Option<(usize, usize)> {
    let mut window: VecDeque<(usize, u64)> = VecDeque::with_capacity(values.len() / 2);
    let mut sum = 0;
    for (i, value) in values.iter().enumerate() {
        window.push_back((i, *value));
        sum += value;

        while sum > target_sum && window.len() > 0 {
            sum -= window.pop_front().unwrap().1;
        }

        if sum == target_sum {
            return Some((window.front().unwrap().0, window.back().unwrap().0 + 1));
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<u64> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut processor = XmasProcessor::new(25);
    for value in &values {
        if !processor.push(*value) {
            println!("First invalid value: {}", value);

            if let Some((lb, ub)) = find_contiguous_range_with_sum(&values, *value) {
                let min = values[lb..ub].iter().min().unwrap();
                let max = values[lb..ub].iter().max().unwrap();
                println!("Encryption weakness: {}", min + max);
            }

            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [u64; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_xmas_processor_push() {
        let mut processor = XmasProcessor::new(5);
        let result: Vec<bool> = INPUT.iter().map(|x| processor.push(*x)).collect();
        assert_eq!(
            result,
            vec![
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                false, true, true, true, true, true
            ]
        );
    }

    #[test]
    fn test_find_contiguous_range_with_sume() {
        assert_eq!(find_contiguous_range_with_sum(&INPUT, 127), Some((2, 6)));
    }
}
