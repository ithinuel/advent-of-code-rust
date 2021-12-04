use aoc_runner_derive::*;
use itertools::Itertools;

type Board = Vec<Vec<(u32, bool)>>;

#[aoc_generator(day4)]
fn gen(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut blocks = input.split("\n\n");
    let random_numbers: Vec<u32> = blocks
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    let boards = blocks
        .map(|b| {
            b.lines()
                .map(|b| {
                    b.split_ascii_whitespace()
                        .filter_map(|n| n.parse().ok())
                        .map(|n| (n, false))
                        .collect()
                })
                .collect()
        })
        .collect();
    (random_numbers, boards)
}

#[aoc(day4, part1)]
fn part1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (rnd, mut boards) = input.clone();
    for &n in rnd.iter() {
        // find n on the board, if present mark it
        for b in boards.iter_mut() {
            let mut set = None;
            'board: for (i, line) in b.iter_mut().enumerate() {
                for (j, digit) in line.iter_mut().enumerate() {
                    if digit.0 == n {
                        digit.1 = true;
                        set = Some((i, j));
                        break 'board;
                    }
                }
            }
            if let Some((i, j)) = set {
                let line = b[i].iter().filter(|n| n.1).count();
                let column = b.iter().map(|line| line[j]).filter(|n| n.1).count();
                if line == 5 || column == 5 {
                    return b
                        .iter()
                        .flat_map(|line| line.iter())
                        .filter_map(|n| if !n.1 { Some(n.0) } else { None })
                        .sum::<u32>()
                        * n;
                }
            }
        }
    }
    0
}

//#[aoc(day4, part2)]
fn part2(input: &(Vec<u32>, Vec<Board>)) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{gen, part1 as solve_part1, part2 as solve_part2};

    const EXAMPLE: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1() {
        assert_eq!(4512, solve_part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        //assert_eq!((), solve_part2(&gen(EXAMPLE)));
    }
}
