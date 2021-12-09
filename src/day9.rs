use aoc_runner_derive::*;
use itertools::Itertools;

const NEIGHBOORS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, cell)| {
                if NEIGHBOORS
                    .iter()
                    .flat_map(|(dx, dy)| {
                        input
                            .get(((y as isize) + dy) as usize)
                            .and_then(|line| line.get(((x as isize) + dx) as usize))
                    })
                    .all(|cell2| cell < cell2)
                {
                    Some(*cell as usize + 1)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn part2(input: &[Vec<u8>]) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1() {
        assert_eq!(15, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(1134, super::part2(&gen(EXAMPLE)));
    }
}
