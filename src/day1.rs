use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<i32> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(i, j)| j - i)
        .filter(|&n| n > 0)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        .zip(input.windows(3).skip(1).map(|w| w.iter().sum()))
        .map(|(i, j): (i32, i32)| j - i)
        .filter(|&n| n > 0)
        .count()
}

#[cfg(test)]
mod test {
    use super::{gen, part1 as solve_part1, part2 as solve_part2};

    const EXAMPLE: &str = r"199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1() {
        assert_eq!(7, solve_part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(5, solve_part2(&gen(EXAMPLE)));
    }
}
