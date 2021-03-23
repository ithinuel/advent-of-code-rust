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
mod test_part1 {
    use super::part1;
    #[test]
    fn test_0_3_6() {
        assert_eq!(part1(&[0, 3, 6]), 436);
    }
    #[test]
    fn test_1_3_2() {
        assert_eq!(part1(&[1, 3, 2]), 1);
    }
    #[test]
    fn test_2_1_3() {
        assert_eq!(part1(&[2, 1, 3]), 10);
    }
    #[test]
    fn test_1_2_3() {
        assert_eq!(part1(&[1, 2, 3]), 27);
    }
    #[test]
    fn test_2_3_1() {
        assert_eq!(part1(&[2, 3, 1]), 78);
    }
    #[test]
    fn test_3_2_1() {
        assert_eq!(part1(&[3, 2, 1]), 438);
    }
    #[test]
    fn test_3_1_2() {
        assert_eq!(part1(&[3, 1, 2]), 1836);
    }
}

#[cfg(all(test, not(debug_assertions)))]
mod test_part2 {
    use super::part2;
    use super::play_n_round;
    #[test]
    fn test_0_3_6_2020() {
        assert_eq!(play_n_round(&[0, 3, 6], 2020), 436);
    }
    #[test]
    fn test_play_2020_round_1_3_2() {
        assert_eq!(play_n_round(&[1, 3, 2], 2020), 1);
    }
    #[test]
    fn test_play_2020_round_2_1_3() {
        assert_eq!(play_n_round(&[2, 1, 3], 2020), 10);
    }
    #[test]
    fn test_play_2020_round_1_2_3() {
        assert_eq!(play_n_round(&[1, 2, 3], 2020), 27);
    }
    #[test]
    fn test_play_2020_round_2_3_1() {
        assert_eq!(play_n_round(&[2, 3, 1], 2020), 78);
    }
    #[test]
    fn test_play_2020_round_3_2_1() {
        assert_eq!(play_n_round(&[3, 2, 1], 2020), 438);
    }
    #[test]
    fn test_play_2020_round_3_1_2() {
        assert_eq!(play_n_round(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_part2_0_3_6() {
        assert_eq!(part2(&[0, 3, 6]), 175594);
    }
    #[test]
    fn test_part2_1_3_2() {
        assert_eq!(part2(&[1, 3, 2]), 2578);
    }
    #[test]
    fn test_part2_2_1_3() {
        assert_eq!(part2(&[2, 1, 3]), 3544142);
    }
    #[test]
    fn test_part2_1_2_3() {
        assert_eq!(part2(&[1, 2, 3]), 261214);
    }
    #[test]
    fn test_part2_2_3_1() {
        assert_eq!(part2(&[2, 3, 1]), 6895259);
    }
    #[test]
    fn test_part2_3_2_1() {
        assert_eq!(part2(&[3, 2, 1]), 18);
    }
    #[test]
    fn test_part2_3_1_2() {
        assert_eq!(part2(&[3, 1, 2]), 362);
    }
}
