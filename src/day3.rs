use aoc_runner_derive::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .filter(|l: &Vec<bool>| !l.is_empty())
        .collect()
}

#[aoc(day3, part1)]
fn part1(forest_pattern: &[Vec<bool>]) -> usize {
    forest_pattern
        .iter()
        .enumerate()
        .filter(|(n, line)| line[(n * 3 % line.len())])
        .count()
}

const SLOPES: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

#[aoc(day3, part2)]
fn part2(forest_pattern: &[Vec<bool>]) -> usize {
    SLOPES
        .iter()
        .map(|&slope| {
            forest_pattern
                .iter()
                .step_by(slope.0)
                .enumerate()
                .filter(|(column, v)| v[(column * slope.1) % v.len()])
                .count()
        })
        .product()
}
