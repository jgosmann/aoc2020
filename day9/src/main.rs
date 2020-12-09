use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

struct XmasProcessor {
    preamble_len: usize,
    window_queue: VecDeque<u32>,
    window_tree: HashSet<u32>,
}

impl XmasProcessor {
    fn new(preamble_len: usize) -> Self {
        XmasProcessor {
            preamble_len,
            window_queue: VecDeque::with_capacity(preamble_len),
            window_tree: HashSet::new(),
        }
    }

    fn push(&mut self, value: u32) -> bool {
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

fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap());
    let mut processor = XmasProcessor::new(25);
    for value in values {
        if !processor.push(value) {
            println!("First invalid value: {}", value);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas_processor_push() {
        let input = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut processor = XmasProcessor::new(5);
        let result: Vec<bool> = input.iter().map(|x| processor.push(*x)).collect();
        assert_eq!(
            result,
            vec![
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                false, true, true, true, true, true
            ]
        );
    }
}
