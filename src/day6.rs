use aoc_runner_derive::*;

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[usize]) -> usize {
    let mut input: Vec<_> = input.to_vec();
    input.sort_unstable();

    for _d in 0..80 {
        let new_cnt = input.iter().enumerate().find(|(_, &n)| n >= 1).unwrap().0;
        if new_cnt > 0 {
            input.drain(0..new_cnt);
        }
        input.iter_mut().for_each(|n| *n -= 1);
        for _ in 0..new_cnt {
            input.push(8);
            input.push(6);
        }
        input.sort_unstable();
    }
    input.len()
}

#[aoc(day6, part2)]
fn part2(input: &[usize]) -> usize {
    let mut pop = [0; 9];
    input.iter().for_each(|&n| {
        pop[n] += 1;
    });

    for _d in 0..256 {
        let n = pop[0];
        pop.rotate_left(1);
        pop[6] += n;
        pop[8] = n;
    }

    pop.iter().cloned().sum()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"3,4,3,1,2";

    #[test]
    fn part1() {
        assert_eq!(5934, super::part1(&gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(26984457539, super::part2(&gen(EXAMPLE)));
    }
}
