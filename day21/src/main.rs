#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::{self, BufRead};

lazy_static! {
    static ref FOOD_PARSE_REGEX: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
}

type Ingredient = String;
type Allergen = String;

#[derive(Debug, PartialEq)]
struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

impl TryFrom<&str> for Food {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if let Some(cap) = FOOD_PARSE_REGEX.captures(input) {
            Ok(Self {
                ingredients: cap[1].split(" ").map(String::from).collect(),
                allergens: cap[2].split(", ").map(String::from).collect(),
            })
        } else {
            Err(())
        }
    }
}

fn count_allergen_free_ingredients(foods: &Vec<Food>) -> usize {
    let mut might_be_contained_in: HashMap<&Allergen, HashSet<&Ingredient>> = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            might_be_contained_in
                .entry(allergen)
                .and_modify(|e| {
                    *e = e
                        .intersection(&food.ingredients.iter().collect())
                        .copied()
                        .collect()
                })
                .or_insert(food.ingredients.iter().collect());
        }
    }

    let mut allergen_free: HashSet<&Ingredient> =
        foods.iter().flat_map(|f| &f.ingredients).collect();
    for ingredients in might_be_contained_in.values() {
        for ingredient in ingredients.iter() {
            allergen_free.remove(ingredient);
        }
    }

    allergen_free
        .iter()
        .flat_map(|allergen| {
            foods.iter().map(move |f| {
                if f.ingredients.contains(allergen) {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let foods: Vec<Food> = stdin
        .lock()
        .lines()
        .map(|line| Food::try_from(line.unwrap().as_str()).unwrap())
        .collect();
    println!("{}", count_allergen_free_ingredients(&foods));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_test_parsing() {
        assert_eq!(
            Food::try_from("ab cd (contains dairy, fish)").unwrap(),
            Food {
                ingredients: vec!["ab".into(), "cd".into()],
                allergens: vec!["dairy".into(), "fish".into()]
            }
        )
    }

    #[test]
    fn test_count_allergen_free_ingredients() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ];
        let foods: Result<Vec<Food>, ()> = input.into_iter().map(Food::try_from).collect();
        let foods = foods.unwrap();
        assert_eq!(count_allergen_free_ingredients(&foods), 5);
    }
}
