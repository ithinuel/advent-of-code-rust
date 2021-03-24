use aoc_runner_derive::*;

#[aoc_generator(day15)]
fn gen(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}

#[aoc(day15, part1)]
fn part1(starting_values: &[usize]) -> usize {
    let mut memory = starting_values.to_vec();
    (starting_values.len()..2020).fold(0, |_, _| {
        let previous = *memory.last().unwrap();
        let last = memory[..memory.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, v)| v == previous)
            .last();

        let new = last.map(|(idx, _)| memory.len() - (idx + 1)).unwrap_or(0);
        memory.push(new);
        new
    })
}

fn play_n_round(starting_values: &[usize], rounds: usize) -> usize {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    starting_values[..starting_values.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .for_each(|(idx, v)| {
            map.insert(v, idx);
        });
    (starting_values.len()..rounds).fold(*starting_values.last().unwrap(), |previous, round| {
        let new = match map.get(&previous) {
            None => 0,
            Some(&last_seen) => round - 1 - last_seen,
        };
        /*println!(
            "{:>3}:{:>3}:{:>5}:{:>3} {:?}",
            round,
            previous,
            map.get(&previous)
                .map_or_else(|| "-".to_string(), |v| format!("{}", v)),
            new,
            map
        );*/
        map.insert(previous, round - 1);
        new
    })
}

#[aoc(day15, part1, alt1)]
fn part1_alt1(starting_values: &[usize]) -> usize {
    play_n_round(starting_values, 2020)
}

#[aoc(day15, part2)]
fn part2(starting_values: &[usize]) -> usize {
    play_n_round(starting_values, 30_000_000)
}

#[cfg(test)]
mod test {
    use super::part1 as solve_part1;
    use super::play_n_round;

    #[cfg(not(debug_assertions))]
    use super::part2 as solve_part2;

    const EXAMPLE: [(&[usize], usize, usize); 7] = [
        (&[0, 3, 6], 436, 175594),
        (&[1, 3, 2], 1, 2578),
        (&[2, 1, 3], 10, 3544142),
        (&[1, 2, 3], 27, 261214),
        (&[2, 3, 1], 78, 6895259),
        (&[3, 2, 1], 438, 18),
        (&[3, 1, 2], 1836, 362),
    ];

    #[test]
    fn part1() {
        for &(input, expect, _) in EXAMPLE.iter() {
            assert_eq!(expect, solve_part1(input));
        }
    }

    #[test]
    fn part1_alt1() {
        for &(input, expect, _) in EXAMPLE.iter() {
            assert_eq!(expect, play_n_round(input, 2020));
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn part2() {
        use rayon::prelude::*;
        EXAMPLE.par_iter().for_each(|&(input, _, expect)| {
            assert_eq!(expect, solve_part2(input));
        });
    }
}
