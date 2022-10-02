use aoc_helper::*;

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<i32> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.sort_unstable();

    let mut acc = 0;
    let median = input
        .iter()
        .find(|_| {
            acc += 1;
            acc >= (input.len() / 2)
        })
        .expect("Not median found");

    input.iter().map(|n| (n - median).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> Option<i32> {
    let max = *input.iter().max()?;
    (0..max)
        .map(|pos| {
            input
                .iter()
                .map(|&n| (n - pos).abs())
                .map(|distance| (distance * (distance + 1)) / 2)
                .sum()
        })
        .min()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1() {
        assert_eq!(37, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(Some(168), super::part2(&gen(EXAMPLE)));
    }
}
