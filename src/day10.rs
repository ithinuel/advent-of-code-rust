use aoc_runner_derive::*;
use either::Either::{self, *};
use itertools::Itertools;

fn check_line(line: &str) -> Result<(), Either<u8, Vec<u8>>> {
    let mut stack = Vec::new();
    for b in line.bytes() {
        if let b'(' | b'<' | b'{' | b'[' = b {
            stack.push(b);
        } else {
            let is_ok = stack.pop().map(|b2| match (b2, b) {
                (b'(', b')') | (b'{', b'}') | (b'[', b']') | (b'<', b'>') => true,
                _ => false,
            });
            if !is_ok.unwrap_or(true) {
                return Err(Left(b));
            }
        }
    }
    if !stack.is_empty() {
        Err(Right(stack))
    } else {
        Ok(())
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| check_line(l).err().and_then(|e| e.left()))
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            check_line(line).err().and_then(|e| e.right()).map(|stack| {
                stack.into_iter().rev().map(|b| match b {
                    b'(' => b')',
                    b'{' => b'}',
                    b'[' => b']',
                    b'<' => b'>',
                    _ => unreachable!(),
                })
            })
        })
        .map(|end| {
            end.fold(0, |acc, b| {
                acc * 5
                    + match b {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect_vec();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1() {
        assert_eq!(26397, super::part1(EXAMPLE));
    }

    #[test]
    fn part2() {
        assert_eq!(288957, super::part2(EXAMPLE));
    }
}
