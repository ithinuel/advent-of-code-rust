use anyhow::Result;
use aoc_runner_derive::*;

#[rustfmt::skip]
const DIRECTIONS: [Coord; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

type Coord = (isize, isize);

fn _print_map(map: &[Vec<char>]) {
    map.iter().for_each(|l| {
        l.iter().for_each(|seat| print!("{}", seat));
        println!();
    });
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn solve(
    map: &[Vec<char>],
    mut when_free: impl FnMut(&[Vec<char>], Coord) -> bool,
    mut when_occupied: impl FnMut(&[Vec<char>], Coord) -> bool,
) -> usize {
    let mut map = map.to_vec();
    loop {
        //println!();
        //_print_map(&map);
        let mut changed = false;
        let new = map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        let coord = (x as isize, y as isize);
                        match seat {
                            'L' if when_free(&map, coord) => {
                                changed = true;
                                '#'
                            }
                            '#' if when_occupied(&map, coord) => {
                                changed = true;
                                'L'
                            }
                            cell => *cell,
                        }
                    })
                    .collect()
            })
            .collect();

        if !changed {
            break;
        }
        map = new;
    }
    map.iter()
        .flat_map(|l| l.iter().copied())
        .filter(|v| *v == '#')
        .count()
}

#[aoc(day11, part1)]
fn part1(map: &[Vec<char>]) -> usize {
    fn adjacents(map: &[Vec<char>], coord: Coord) -> impl Iterator<Item = char> + '_ {
        DIRECTIONS
            .iter()
            .filter_map(move |&(dx, dy)| {
                let x = coord.0 + dx;
                let y = coord.1 + dy;
                if x < 0 || y < 0 {
                    None
                } else {
                    map.get(y as usize)?.get(x as usize).copied()
                }
            })
            .filter(|&cell| cell != '.')
    }
    solve(
        map,
        |map, coord| adjacents(map, coord).all(|v| v == 'L'),
        |map, coord| adjacents(map, coord).filter(|&v| v == '#').count() >= 4,
    )
}
#[aoc(day11, part2)]
fn part2(map: &[Vec<char>]) -> usize {
    fn adjacents(map: &[Vec<char>], coord: Coord) -> impl Iterator<Item = char> + '_ {
        DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
            (1..)
                .map(|distance| {
                    let x = coord.0 + distance * dx;
                    let y = coord.1 + distance * dy;
                    if x < 0 || y < 0 {
                        None
                    } else {
                        map.get(y as usize)?.get(x as usize).copied()
                    }
                })
                .find(|&cell| cell != Some('.'))
                .flatten()
        })
    }

    solve(
        map,
        |map, coord| adjacents(map, coord).all(|v| v == 'L'),
        |map, coord| adjacents(map, coord).filter(|&v| v == '#').count() >= 5,
    )
}

#[cfg(test)]
mod test {
    use super::{gen, part1, part2};

    const EXAMPLE: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    #[test]
    fn solve_part1() {
        let v = gen(EXAMPLE);
        assert_eq!(37, part1(&v))
    }

    #[test]
    fn solve_part2() {
        let v = gen(EXAMPLE);
        assert_eq!(26, part2(&v))
    }
}
