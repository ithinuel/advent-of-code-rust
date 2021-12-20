use std::collections::HashSet;

use aoc_runner_derive::*;
use itertools::Itertools;

fn find_boundaries(image: &HashSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    image.iter().fold(
        (
            (isize::max_value(), isize::max_value()),
            (isize::min_value(), isize::min_value()),
        ),
        |(top_left, bottom_right), &(x, y)| {
            (
                (isize::min(top_left.0, x), isize::min(top_left.1, y)),
                (isize::max(bottom_right.0, x), isize::max(bottom_right.1, y)),
            )
        },
    )
}

#[aoc_generator(day20)]
fn gen(input: &str) -> (Vec<bool>, HashSet<(isize, isize)>) {
    let (conversion_table, image) = input.split("\n\n").collect_tuple().unwrap();
    let conversion_table = conversion_table.bytes().map(|b| b == b'#').collect_vec();
    let image: HashSet<_> = image
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.bytes()
                .enumerate()
                .filter_map(move |(col, b)| (b == b'#').then(move || (row as isize, col as isize)))
        })
        .collect();
    (conversion_table, image)
}

fn enhance(
    table: &[bool],
    image: HashSet<(isize, isize)>,
    is_outside_lit: bool,
) -> HashSet<(isize, isize)> {
    let ((xmin, ymin), (xmax, ymax)) = find_boundaries(&image);

    (xmin - 1..=xmax + 1)
        .cartesian_product(ymin - 1..=ymax + 1)
        .filter_map(|(x, y)| {
            let idx = (-1..=1)
                .cartesian_product(-1..=1)
                .map(|(xp, yp)| {
                    let (x, y) = (x + xp, y + yp);
                    (if !(xmin..=xmax).contains(&x) || !(ymin..=ymax).contains(&y) {
                        is_outside_lit
                    } else {
                        image.contains(&(x, y))
                    }) as usize
                })
                .fold(0, |acc, b| acc << 1 | b);
            table[idx].then(|| (x, y))
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1((table, image): &(Vec<bool>, HashSet<(isize, isize)>)) -> usize {
    let image = image.to_owned();
    let image = enhance(table, image, false);
    let image = enhance(table, image, table[0]);
    image.len()
}

#[aoc(day20, part2)]
fn part2((table, image): &(Vec<bool>, HashSet<(isize, isize)>)) -> usize {
    let mut image = image.to_owned();
    let mut outside_is_lit = false;
    for _ in 0..50 {
        image = enhance(table, image, outside_is_lit);
        outside_is_lit ^= table[0];
    }
    image.len()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part1() {
        assert_eq!(35, super::part1(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(3351, super::part2(&super::gen(EXAMPLE)));
    }
}
