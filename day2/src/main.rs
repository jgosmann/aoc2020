use std::io::{self, BufRead};

type Password = String;

trait Policy {
    fn parse(definition: &str) -> Self;
    fn check_password(&self, password: &Password) -> bool;
}

struct OldPolicy {
    character: char,
    occurences_bounds: (usize, usize),
}

impl OldPolicy {
    fn parse(definition: &str) -> Self {
        let split: Vec<&str> = definition.splitn(2, ' ').collect();
        let character = split[1].chars().next().unwrap();
        let split: Vec<&str> = split[0].splitn(2, '-').collect();
        let lower_bound = split[0].parse().unwrap();
        let upper_bound = split[1].parse().unwrap();

        Self {
            character,
            occurences_bounds: (lower_bound, upper_bound),
        }
    }

    fn check_password(&self, password: &Password) -> bool {
        let count = password.chars().filter(|c| *c == self.character).count();
        self.occurences_bounds.0 <= count && count <= self.occurences_bounds.1
    }
}

struct NewPolicy {
    character: char,
    indices: Vec<usize>,
}

impl NewPolicy {
    fn parse(definition: &str) -> Self {
        let split: Vec<&str> = definition.splitn(2, ' ').collect();
        let character = split[1].chars().next().unwrap();
        let indices = split[0]
            .split('-')
            .map(str::parse)
            .map(Result::unwrap)
            .map(|i: usize| i - 1)
            .collect();
        Self { character, indices }
    }

    fn check_password(&self, password: &Password) -> bool {
        let mut password_iter = password.chars();
        let raw_result = self.indices.iter().fold(
            (0, 0),
            |(pos_in_password, matching_characters): (usize, usize), i| {
                let advance_by = i - pos_in_password;
                let c = password_iter.nth(advance_by).unwrap();
                if c == self.character {
                    (i + 1, matching_characters + 1)
                } else {
                    (i + 1, matching_characters)
                }
            },
        );

        raw_result.1 == 1
    }
}

fn parse_input_line(line: &str) -> (OldPolicy, NewPolicy, Password) {
    let split: Vec<&str> = line.splitn(2, ':').collect();
    let policy_definition = split[0];
    let password = split[1].trim();
    return (
        OldPolicy::parse(policy_definition),
        NewPolicy::parse(policy_definition),
        String::from(password),
    );
}

fn main() {
    let mut valid_old = 0;
    let mut valid_new = 0;
    io::stdin()
        .lock()
        .lines()
        .map(|l| parse_input_line(&l.unwrap()))
        .for_each(|(old_policy, new_policy, password)| {
            if old_policy.check_password(&password) {
                valid_old += 1;
            }
            if new_policy.check_password(&password) {
                valid_new += 1;
            }
        });
    println!("Valid for old policy: {}", valid_old);
    println!("Valid for new policy: {}", valid_new);
}
