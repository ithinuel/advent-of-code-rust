use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<usize> {
    let mut adapters: Vec<_> = input.lines().filter_map(|s| s.parse().ok()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    adapters
}

#[aoc(day10, part1)]
fn part1(adapters: &[usize]) -> usize {
    let (ones, threes) = adapters
        .iter()
        .tuple_windows()
        .fold((0, 0), |(ones, threes), (a, b)| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });

    ones * (threes + 1)
}
#[aoc(day10, part2)]
fn part2(adapters: &[usize]) -> u64 {
    use std::collections::BTreeMap;
    let mut edges = BTreeMap::new();
    adapters.iter().for_each(|&a| {
        edges.entry(a).or_insert_with(Vec::new).extend(
            (1..4)
                .map(|n| a + n)
                .filter(|b| adapters.binary_search(b).is_ok()),
        );
    });

    // The input is a Directed Acyclic Graph
    // count number of path from 0 to max
    let mut paths_count = BTreeMap::new();
    adapters.iter().rev().for_each(|&v| {
        let neighbours = &edges[&v];
        if neighbours.is_empty() {
            paths_count.insert(v, 1u64);
        } else {
            paths_count.insert(v, neighbours.iter().map(|n| paths_count[n]).sum());
        }
    });

    paths_count[&0]
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE2: &str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    const EXAMPLE_AS_ARRAY: &[usize] = &[0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    const EXAMPLE_AS_ARRAY2: &[usize] = &[
        0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49,
    ];

    #[test]
    fn gen() {
        assert_eq!(EXAMPLE_AS_ARRAY, &super::gen(EXAMPLE));
        assert_eq!(EXAMPLE_AS_ARRAY2, &super::gen(EXAMPLE2));
    }

    #[test]
    fn part1() {
        assert_eq!(7 * 5, super::part1(EXAMPLE_AS_ARRAY));
        assert_eq!(22 * 10, super::part1(EXAMPLE_AS_ARRAY2));
    }

    #[test]
    fn part2() {
        assert_eq!(8, super::part2(EXAMPLE_AS_ARRAY));
        assert_eq!(19208, super::part2(EXAMPLE_AS_ARRAY2));
    }
}
