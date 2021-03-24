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

#[cfg(test)]
mod test {
    use super::{gen, part1 as solve_part1, part2 as solve_part2};
    const EXAMPLE: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part1() {
        assert_eq!(2, solve_part1(&gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(1, solve_part2(&gen(EXAMPLE)));
    }
}
