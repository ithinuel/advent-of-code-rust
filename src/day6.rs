use aoc_runner_derive::*;

#[aoc_generator(day6, part1, bruteforce)]
fn gen_part1_bruteforce(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day6, part1, bruteforce)]
fn part1_bruteforce(input: &[usize]) -> usize {
    let mut input = input.to_owned();

    for _d in 0..80 {
        let len = input.len();
        input = input.drain(..).filter(|&n| n > 0).map(|n| n - 1).collect();
        input.extend(std::iter::repeat([6, 8]).take(len - input.len()).flatten());
    }
    input.len()
}

#[aoc_generator(day6)]
fn gen(input: &str) -> [usize; 9] {
    let mut pop = [0; 9];
    input
        .split(',')
        .filter_map(|n| n.parse().ok())
        .for_each(|n: usize| {
            pop[n] += 1;
        });
    pop
}

fn one_generation(pop: &mut [usize; 9]) {
    let n = pop[0];
    pop.rotate_left(1);
    pop[6] += n;
    pop[8] = n;
}

#[aoc(day6, part1)]
fn part1(input: &[usize; 9]) -> usize {
    let mut pop = [0; 9];
    pop.copy_from_slice(input);
    for _ in 0..80 {
        one_generation(&mut pop);
    }

    pop.iter().cloned().sum()
}

#[aoc(day6, part2)]
fn part2(input: &[usize; 9]) -> usize {
    let mut pop = [0; 9];
    pop.copy_from_slice(input);
    for _ in 0..256 {
        one_generation(&mut pop);
    }

    pop.iter().cloned().sum()
}

#[aoc(day6, part1, no_rotation)]
fn part1_no_rotation(input: &[usize; 9]) -> usize {
    let mut pop = [0; 9];
    pop.copy_from_slice(input);
    for day in 0..80 {
        pop[(day + 7) % 9] += pop[day % 9];
    }

    pop.iter().cloned().sum()
}
#[aoc(day6, part2, no_rotation)]
fn part2_no_rotation(input: &[usize; 9]) -> usize {
    let mut pop = [0; 9];
    pop.copy_from_slice(input);
    for day in 0..256 {
        pop[(day + 7) % 9] += pop[day % 9];
    }

    pop.iter().cloned().sum()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"3,4,3,1,2";

    use super::{gen, gen_part1_bruteforce};

    #[test]
    fn part1_bruteforce() {
        assert_eq!(
            5934,
            super::part1_bruteforce(&gen_part1_bruteforce(EXAMPLE))
        );
    }
    #[test]
    fn part1() {
        assert_eq!(5934, super::part1(&gen(EXAMPLE)));
    }
    #[test]
    fn part1_no_rotation() {
        assert_eq!(5934, super::part1_no_rotation(&gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(26984457539, super::part2(&gen(EXAMPLE)));
    }
    #[test]
    fn part2_no_rotation() {
        assert_eq!(26984457539, super::part2_no_rotation(&gen(EXAMPLE)));
    }
}
