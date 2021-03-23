use aoc_runner_derive::*;
use itertools::Itertools;

type InputType = (usize, usize, char, String);

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|s| {
            let (rule, pwd) = s.split(": ").next_tuple().unwrap();
            let (range, c) = rule.split(' ').next_tuple().unwrap();
            let (a, b) = range
                .split('-')
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .next_tuple()
                .unwrap();
            (a, b, c.chars().next().unwrap(), pwd.to_string())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[InputType]) -> usize {
    input
        .iter()
        .filter(|(min, max, ch, pwd)| {
            let occurences = pwd.chars().filter(|c| c == ch).count();
            (*min..=*max).contains(&occurences)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[InputType]) -> usize {
    input
        .iter()
        .filter(|(first, next, ch, pwd)| {
            let is_eq_to_ch = |c| c == *ch;
            let first = pwd.chars().nth(first - 1).map(is_eq_to_ch).unwrap_or(false);
            let next = pwd.chars().nth(next - 1).map(is_eq_to_ch).unwrap_or(false);
            first ^ next
        })
        .count()
}
