use std::collections::{HashMap, HashSet};

use nom::{bytes::complete::tag, character::complete::alpha1, multi::separated_list1, IResult};

pub type Allergen<'a> = &'a str;
pub type Ingredient<'a> = &'a str;

fn parse_allergens(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(" "), alpha1)(input)
}

pub fn parse_row(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, ingredients) = parse_ingredients(input)?;
    let (input, _) = tag(" (contains ")(input)?;
    let (input, allergens) = parse_allergens(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (ingredients, allergens)))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list1(tag("\n"), parse_row)(input)
}

pub fn map_of_possible_ingredients<'a> (
    input: &'a [(Vec<&str>, Vec<&str>)],
) -> HashMap<Allergen<'a>, HashSet<Ingredient<'a>>> {
    let mut hm: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for (ingredients, allergens) in input {
        let mut hs : HashSet<&str> = HashSet::new();

        for ingredient in ingredients {
            hs.insert(ingredient);
        }

        for allergen in allergens {
            match hm.get(allergen) {
                Some(stored_hash_set) => {
                    let merged = stored_hash_set.intersection(&hs).copied().collect();
                    hm.insert(allergen, merged);
                }
                None => {
                    hm.insert(allergen, hs.clone());
                }
            }
        }
    }

    hm
}

pub fn ingredients_with_possible_allergens<'a>(
    map: &'a HashMap<Allergen<'a>, HashSet<Ingredient<'a>>>,
) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for ingredients in map.values() {
        hs = hs.union(&ingredients).cloned().collect();
    }

    hs
}

pub fn ingredients_definetely_without_allergens<'a>(
    map: &'a HashMap<Allergen, HashSet<Ingredient>>,
    all_ingredients: &'a HashSet<&str>,
) -> HashSet<&'a str> {
    let unsafe_ingredients = ingredients_with_possible_allergens(map);

    all_ingredients
        .difference(&unsafe_ingredients)
        .cloned()
        .collect()
}

pub fn all_ingredients<'a> (input: &'a [(Vec<&str>, Vec<&str>)]) -> HashSet<Ingredient<'a>> {
    let mut hs: HashSet<Ingredient> = HashSet::new();
    for (ingredients, _) in input {
        for ingredient in ingredients {
            hs.insert(ingredient);
        }
    }

    hs
}

pub fn solve(input: &str) -> String {
    let (_, rows) = parse_input(input).unwrap();

    let all_ingredients = all_ingredients(&rows);
    let hm = map_of_possible_ingredients(&rows);

    let safe_ingredients = ingredients_definetely_without_allergens(&hm, &all_ingredients);

    safe_ingredients.into_iter().map(|si| {
        rows.iter().filter(|(ingredients, _)| ingredients.contains(&si)).count()
    }).sum::<usize>().to_string()
}
