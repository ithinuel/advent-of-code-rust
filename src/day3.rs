use aoc_runner_derive::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .filter(|l: &Vec<bool>| !l.is_empty())
        .collect()
}

#[aoc(day3, part1)]
fn part1(forest_pattern: &[Vec<bool>]) -> usize {
    forest_pattern
        .iter()
        .enumerate()
        .filter(|(n, line)| line[(n * 3 % line.len())])
        .count()
}

const SLOPES: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

#[aoc(day3, part2)]
fn part2(forest_pattern: &[Vec<bool>]) -> usize {
    SLOPES
        .iter()
        .map(|&slope| {
            forest_pattern
                .iter()
                .step_by(slope.0)
                .enumerate()
                .filter(|(column, v)| v[(column * slope.1) % v.len()])
                .count()
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::gen;
    use super::part1 as solve_part1;
    use super::part2 as solve_part2;

    const EXAMPLE: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    const BITVEC: [u16; 11] = [
        0b000_0000_1100,
        0b001_0001_0001,
        0b010_0100_0010,
        0b101_0001_0100,
        0b010_0110_0010,
        0b000_0011_0100,
        0b100_0010_1010,
        0b100_0000_0010,
        0b000_1000_1101,
        0b100_0011_0001,
        0b101_0001_0010,
    ];
    fn example_as_vec() -> Vec<Vec<bool>> {
        BITVEC
            .iter()
            .map(|v| (0..11).map(|i| ((v >> i) & 1) == 1).collect())
            .collect()
    }

    #[test]
    fn generator() {
        assert_eq!(example_as_vec(), gen(EXAMPLE));
    }
    #[test]
    fn part1() {
        let input = example_as_vec();
        assert_eq!(7, solve_part1(&input));
    }
    #[test]
    fn part2() {
        let input = example_as_vec();
        assert_eq!(336, solve_part2(&input));
    }
}
