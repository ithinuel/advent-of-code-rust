use std::collections::HashMap;

use aoc_runner_derive::*;
use bimap::BiMap;
use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

#[aoc_generator(day21)]
fn gen(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .par_lines()
        .filter_map(|l| {
            let (ingredients, allergens) = l.split(" (contains ").collect_tuple()?;
            let ingredients = ingredients.split(' ').map(String::from).collect_vec();
            let allergens = allergens
                .trim_end_matches(')')
                .split(", ")
                .map(String::from)
                .collect_vec();
            Some((ingredients, allergens))
        })
        .collect()
}

fn find_allergens(recipes: &[(Vec<String>, Vec<String>)]) -> BiMap<&str, &str> {
    let mut pair_occurrences = HashMap::new();

    recipes
        .iter()
        .flat_map(|(ingrs, allers)| ingrs.iter().cartesian_product(allers.iter()))
        .map(|(ingr, aller)| (ingr.as_str(), aller.as_str()))
        .for_each(|(ingr, aller)| {
            *pair_occurrences.entry((ingr, aller)).or_insert(0) += 1;
        });

    let aller_occurences = recipes
        .iter()
        .flat_map(|(_, allers)| allers.iter().map(String::as_str))
        .counts();

    let mut bimap = BiMap::new();

    loop {
        let possible_matches: HashMap<_, _> = pair_occurrences
            .iter()
            // only consider the ingredient-allergen pair that appear ass often as the alergen them
            // selves
            .filter(|&(k, v)| aller_occurences[k.1] == *v)
            // ignore those that are already matched
            .filter(|(k, _)| !(bimap.contains_left(k.0) || bimap.contains_right(k.1)))
            .collect();

        // if none's left, we matched them all !
        if possible_matches.is_empty() {
            break;
        }

        // count in how many pair each ingredient appears
        let counts: HashMap<_, _> = possible_matches
            .iter()
            .counts_by(|((ingr, _), _)| *ingr)
            .into_iter()
            .group_by(|&(_, cnt)| cnt)
            .into_iter()
            .map(|(cnt, group)| (cnt, group.map(|(aller, _)| aller).collect_vec()))
            .collect();
        // pick from possible match the ingredients appearing only once.
        let known = counts.get(&1).expect("Undeterministic resolution");
        bimap.extend(
            possible_matches
                .into_iter()
                .filter_map(|(&k, _)| known.contains(&k.0).then(|| k)),
        )
    }
    bimap
}

#[aoc(day21, part1)]
fn part1(recipes: &[(Vec<String>, Vec<String>)]) -> usize {
    let allergens = find_allergens(recipes);
    recipes
        .iter()
        .flat_map(|(ingr, _)| ingr.iter())
        .filter(|ingr| !allergens.contains_left(ingr.as_str()))
        .count()
}

#[aoc(day21, part2)]
fn part2(recipes: &[(Vec<String>, Vec<String>)]) -> String {
    let allergens = find_allergens(recipes);
    allergens
        .into_iter()
        .sorted_by_key(|&(_, v)| v)
        .map(|(k, _)| k)
        .join(",")
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn part1() {
        assert_eq!(5, super::part1(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", &super::part2(&super::gen(EXAMPLE)));
    }
}
