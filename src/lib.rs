use std::{
    collections::{BTreeSet, HashSet},
    ops::RangeInclusive,
};

use itertools::Itertools;
use yaah::{aoc, aoc_generator, aoc_lib, aoc_year};

aoc_year!(2022);

#[aoc(day1, part1)]
fn day1_part1(input: &'static str) -> Option<usize> {
    input
        .split("\n\n")
        .map(|s| s.split('\n').filter_map(|v| v.parse::<usize>().ok()).sum())
        .max()
}

#[aoc(day1, part2)]
fn day1_part2(input: &'static str) -> usize {
    input
        .split("\n\n")
        .map(|s| -> usize { s.split('\n').filter_map(|v| v.parse::<usize>().ok()).sum() })
        .sorted_unstable()
        .rev()
        .take(3)
        .sum::<usize>()
}

#[aoc(day2, part1)]
fn day2_part1(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" | "B Y" | "C Z" => 3,
            "C X" | "A Y" | "B Z" => 6,
            _ => 0,
        } + match line.chars().last() {
           Some('X') => 1,
           Some('Y') => 2,
           Some('Z') => 3,
           _ => unreachable!(),
        })
        .sum()
}

#[aoc(day2, part2)]
fn day2_part2(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3, // Scissors loss against rock
            "B X" => 1, // Rock loss against paper
            "C X" => 2, // Paper loss against scissors
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "A Z" => 8, // Paper wins on rock
            "B Z" => 9, // Scissors wins on paper
            "C Z" => 7, // Rock wins on scissors
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day3, part1)]
fn day3_part1(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let compartiment_size = line.len() / 2;
            let (first, second) = line.split_at(compartiment_size);
            let left: HashSet<_> = first.chars().collect();
            let right: HashSet<_> = second.chars().collect();
            let miss_placed = left.intersection(&right).collect_vec();
            assert_eq!(
                1,
                miss_placed.len(),
                "only one item type should appear in both"
            );
            miss_placed.first().map(|v| **v)
        })
        .map(|c| {
            (if c.is_ascii_uppercase() {
                (c as u8) - b'A' + 27
            } else {
                (c as u8) - b'a' + 1
            }) as usize
        })
        .sum()
}
#[aoc(day3, part2)]
fn day3_part2(input: &'static str) -> usize {
    input
        .lines()
        .tuples()
        .filter_map(|(a, b, c)| {
            let a: HashSet<_> = a.chars().collect();
            let b: HashSet<_> = b.chars().collect();
            let c: HashSet<_> = c.chars().collect();
            let a_b: HashSet<_> = a.intersection(&b).cloned().collect();
            a_b.intersection(&c).cloned().last()
        })
        .map(|c| {
            (if c.is_ascii_uppercase() {
                (c as u8) - b'A' + 27
            } else {
                (c as u8) - b'a' + 1
            }) as usize
        })
        .sum()
}

type Day4List = std::iter::FilterMap<
    std::str::Lines<'static>,
    fn(&str) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>)>,
>;

#[aoc_generator(day4)]
fn day4(input: &'static str) -> Day4List {
    input.lines().filter_map(|l| {
        l.split(',')
            .filter_map(|ranges| {
                let (a, b): (usize, _) = ranges
                    .split('-')
                    .filter_map(|v| v.parse().ok())
                    .collect_tuple()?;
                Some(a..=b)
            })
            .collect_tuple()
    })
}

#[aoc(day4, part1)]
fn day4_part1(input: &Day4List) -> usize {
    input
        .clone()
        .filter(|(a, b)| {
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count()
}

#[aoc(day4, part2)]
fn day4_part2(input: &Day4List) -> usize {
    input
        .clone()
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}

pub type Stacks = Vec<Vec<char>>;
pub type Instructions = Vec<(usize, usize, usize)>;
#[aoc_generator(day5)]
fn day5(input: &'static str) -> Option<(Stacks, Instructions)> {
    let (stacks, instructions) = input.split("\n\n").collect_tuple()?;
    let mut stacks = stacks
        .lines()
        .map(|l| l.chars().skip(1).step_by(4).collect_vec())
        .collect_vec();
    stacks.pop();
    let stack_count = stacks[0].len();
    let mut iters = stacks
        .into_iter()
        .rev()
        .map(|l| l.into_iter())
        .collect_vec();

    stacks = (0..stack_count)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|it| it.next())
                .filter(|&c| c != ' ')
                .collect_vec()
        })
        .collect_vec();

    let instructions = instructions
        .lines()
        .filter_map(|line| {
            line.split(' ')
                .skip(1)
                .step_by(2)
                .filter_map(|v| v.parse::<usize>().ok())
                .collect_tuple::<(_, _, _)>()
        })
        .collect_vec();

    Some((stacks, instructions))
}

#[aoc(day5, part1)]
fn day5_part1(input: &(Stacks, Instructions)) -> String {
    let mut stacks = input.0.clone();
    input.1.iter().for_each(|&(count, from, to)| {
        for _ in 0..count {
            let top = stacks[from - 1]
                .pop()
                .expect("empty stack, nothing to move");
            stacks[to - 1].push(top);
        }
    });

    stacks.iter().filter_map(|s| s.last()).collect()
}

#[aoc(day5, part2)]
fn day5_part2(input: &(Stacks, Instructions)) -> String {
    let mut stacks = input.0.clone();
    input.1.iter().for_each(|&(count, from, to)| {
        let from_len = stacks[from - 1].len() - count;
        let tail = stacks[from - 1].split_off(from_len);
        stacks[to - 1].extend(tail);
        stacks[from - 1].truncate(from_len);
    });

    stacks.iter().filter_map(|s| s.last()).collect()
}

#[aoc(day6, part1)]
fn day6_part1(input: &'static str) -> Option<usize> {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .find_map(|(idx, (a, b, c, d))| {
            (a != b && a != c && a != d && b != c && b != d && c != d).then_some(idx + 4)
        })
}

#[aoc(day6, part2)]
fn day6_part2(input: &'static str) -> Option<usize> {
    let chars = input.chars().collect_vec();
    chars.windows(14).enumerate().find_map(|(idx, arr)| {
        (arr.iter().cloned().collect::<BTreeSet<_>>().len() == 14).then_some(idx + 14)
    })
}
#[aoc(day6, part2, position)]
fn day6_part2_position(input: &'static str) -> Option<usize> {
    input
        .as_bytes()
        .windows(14)
        .position(|arr| arr.iter().collect::<BTreeSet<_>>().len() == 14)
        .map(|v| v + 14)
}
#[aoc(day6, part2, mutable)]
fn day6_part2_mutable(input: &'static str) -> Option<usize> {
    input
        .as_bytes()
        .windows(14)
        .map(|arr| {
            let mut vec = arr.to_owned();
            vec.sort_unstable();
            vec.dedup();
            vec.len()
        })
        .position(|arr_len| arr_len == 14)
        .map(|v| v + 14)
}
#[aoc(day6, part2, mutable_smallvec)]
fn day6_part2_mutable_smallvec(input: &'static str) -> Option<usize> {
    input
        .as_bytes()
        .windows(14)
        .map(|arr| {
            let mut vec: smallvec::SmallVec<[u8; 14]> = smallvec::SmallVec::from_slice(arr);
            vec.sort_unstable();
            vec.dedup();
            vec.len()
        })
        .position(|arr_len| arr_len == 14)
        .map(|v| v + 14)
}

mod day7;

#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1() {
        assert_eq!(15, super::day2_part1("A Y\nB X\nC Z"))
    }
    #[test]
    fn day2_part2() {
        assert_eq!(12, super::day2_part2("A Y\nB X\nC Z"))
    }

    const DAY3: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day3_part1() {
        assert_eq!(157, super::day3_part1(DAY3));
    }

    #[test]
    fn day3_part2() {
        assert_eq!(70, super::day3_part2(DAY3));
    }

    const DAY4: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn day4_part1() {
        assert_eq!(2, super::day4_part1(&super::day4(DAY4)));
    }

    #[test]
    fn day4_part2() {
        assert_eq!(4, super::day4_part2(&super::day4(DAY4)));
    }

    const DAY5: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn day5_part1() {
        assert_eq!(
            "CMZ",
            super::day5_part1(super::day5(DAY5).as_ref().unwrap())
        );
    }

    #[test]
    fn day6_part1() {
        assert_eq!(Some(7), super::day6_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), super::day6_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), super::day6_part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(
            Some(10),
            super::day6_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(11),
            super::day6_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
    #[test]
    fn day6_part2() {
        assert_eq!(
            Some(19),
            super::day6_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(Some(23), super::day6_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), super::day6_part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(
            Some(29),
            super::day6_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            super::day6_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
    #[test]
    fn day6_part2_position() {
        assert_eq!(
            Some(19),
            super::day6_part2_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_position("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_position("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            Some(29),
            super::day6_part2_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            super::day6_part2_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
    #[test]
    fn day6_part2_mutable() {
        assert_eq!(
            Some(19),
            super::day6_part2_mutable("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_mutable("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_mutable("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            Some(29),
            super::day6_part2_mutable("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            super::day6_part2_mutable("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
    #[test]
    fn day6_part2_mutable_smallvec() {
        assert_eq!(
            Some(19),
            super::day6_part2_mutable_smallvec("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_mutable_smallvec("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            Some(23),
            super::day6_part2_mutable_smallvec("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            Some(29),
            super::day6_part2_mutable_smallvec("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            super::day6_part2_mutable_smallvec("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
}

aoc_lib!(with_benchmarks);
