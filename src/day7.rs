use aoc_runner_derive::*;

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();

    let mut acc = 0;
    let median = input
        .iter()
        .find(|_| {
            acc += 1;
            acc >= (input.len() / 2)
        })
        .unwrap();

    input
        .iter()
        .map(|n| if n > median { n - median } else { median - n })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[usize]) -> usize {
    let max = input.iter().max().unwrap();
    (0..*max)
        .map(|pos| {
            input
                .iter()
                .map(|&n| if n > pos { n - pos } else { pos - n })
                .map(|distance| (distance * (distance + 1)) / 2)
                .sum()
        })
        .min()
        .unwrap()
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
        assert_eq!(168, super::part2(&gen(EXAMPLE)));
    }
}
