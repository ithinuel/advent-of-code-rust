use aoc_runner_derive::*;
use itertools::Itertools;

mod remap;
use remap::*;

#[aoc_generator(day19)]
fn gen(input: &str) -> Vec<Report> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .filter_map(|line| {
                    line.split(',')
                        .filter_map(|n| n.parse().ok())
                        .collect_tuple()
                })
                .collect()
        })
        .collect()
}

#[aoc(day19, part1)]
fn part1(input: &[Report]) -> usize {
    rebuild_map(input)
        .into_iter()
        .filter(|(_, obj)| obj.is_beacon())
        .count()
}

#[aoc(day19, part1, faster)]
fn part1_faster(input: &[Report]) -> usize {
    rebuild_map_faster(input)
        .into_iter()
        .filter(|(_, obj)| obj.is_beacon())
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &[Report]) -> Option<i32> {
    let scanners = rebuild_map(input)
        .into_iter()
        .filter_map(|(key, obj)| obj.is_scanner().then(|| key))
        .collect_vec();
    scanners
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        .max()
}
#[aoc(day19, part2, faster)]
fn part2_faster(input: &[Report]) -> Option<i32> {
    let scanners = rebuild_map_faster(input)
        .into_iter()
        .filter_map(|(key, obj)| obj.is_scanner().then(|| key))
        .collect_vec();
    scanners
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        .max()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = include_str!("day19/example.txt");

    #[test]
    fn gen() {
        let res = super::gen(EXAMPLE);
        assert_eq!(5, res.len());
        [
            (404, -588, -901),
            (528, -643, 409),
            (-838, 591, 734),
            (390, -675, -793),
        ]
        .iter()
        .for_each(|coords| assert!(res[0].contains(coords)));
    }

    #[test]
    fn part1() {
        assert_eq!(79, super::part1(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part1_faster() {
        assert_eq!(79, super::part1_faster(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(Some(3621), super::part2(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part2_faster() {
        assert_eq!(Some(3621), super::part2_faster(&super::gen(EXAMPLE)));
    }
}
