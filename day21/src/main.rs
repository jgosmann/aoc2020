#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::rc::Rc;

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

fn find_what_ingredients_an_allergen_might_be_contained_in(
    foods: &Vec<Food>,
) -> HashMap<&Allergen, HashSet<&Ingredient>> {
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
    might_be_contained_in
}

fn count_allergen_free_ingredients(foods: &Vec<Food>) -> usize {
    let might_be_contained_in = find_what_ingredients_an_allergen_might_be_contained_in(foods);

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

fn canonical_dangerous_ingredient_list(foods: &Vec<Food>) -> Vec<Ingredient> {
    use graph::DirectedGraph;

    let might_be_contained_in = find_what_ingredients_an_allergen_might_be_contained_in(foods);

    let mut graph: DirectedGraph<&String> = DirectedGraph::new();
    let start_token = String::from("start");
    let end_token = String::from("end");
    let start = Rc::new(&start_token);
    let end = Rc::new(&end_token);

    for item in might_be_contained_in.iter() {
        let (allergen, ingredients) = item;
        let allergen = Rc::new(*allergen);
        graph.add_edge(&start, &allergen);
        for ingredient in ingredients {
            let ingredient = Rc::new(*ingredient);
            graph.add_edge(&allergen, &ingredient);
            graph.add_edge(&ingredient, &end);
        }
    }

    let flow = graph.max_flow(&start, &end);

    let mut allergens: Vec<&Allergen> = might_be_contained_in.keys().copied().collect();
    allergens.sort_unstable();
    allergens
        .iter()
        .map(|allergen| (**flow.adjancency[allergen].iter().next().unwrap()).clone())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let foods: Vec<Food> = stdin
        .lock()
        .lines()
        .map(|line| Food::try_from(line.unwrap().as_str()).unwrap())
        .collect();
    println!("{}", count_allergen_free_ingredients(&foods));
    println!("{}", canonical_dangerous_ingredient_list(&foods).join(","));
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

    #[test]
    fn test_canonical_dangerous_ingredients() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ];
        let foods: Result<Vec<Food>, ()> = input.into_iter().map(Food::try_from).collect();
        let foods = foods.unwrap();
        assert_eq!(
            canonical_dangerous_ingredient_list(&foods),
            vec![
                String::from("mxmxvkd"),
                String::from("sqjhc"),
                String::from("fvjkl")
            ]
        );
    }
}
