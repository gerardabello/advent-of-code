use std::collections::HashSet;

use crate::d21p1::{map_of_possible_ingredients, parse_input, Allergen, Ingredient};

pub fn solve(input: &str) -> String {
    let (_, rows) = parse_input(input).unwrap();

    let hm = map_of_possible_ingredients(&rows);

    let mut vec_allergen_ingredients: Vec<(Allergen, HashSet<Ingredient>)> =
        hm.into_iter().collect();
    let mut solution: Vec<(Allergen, Ingredient)> = Vec::new();

    for _ in 0..vec_allergen_ingredients.len() {
        // Sort by allergens with less options
        vec_allergen_ingredients.sort_by(|(_, ma), (_, mb)| ma.len().cmp(&mb.len()));

        // Get the first in the list
        let (allergen, ingredients) = vec_allergen_ingredients.drain(..1).next().unwrap();

        // pick first one
        let ingredient: Ingredient = *(ingredients.iter().next().unwrap());

        // add to solution
        solution.push((allergen, ingredient));

        // remove picked ingredient from all other allergens
        for (_, is) in &mut vec_allergen_ingredients {
            is.remove(ingredient);
        }
    }

    // Sort alphabetically by allergen
    solution.sort_by(|(a1, _), (a2, _)| a1.cmp(&a2));

    solution
        .into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<&str>>()
        .join(",")
}
