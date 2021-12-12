use aoc_runner_derive::*;
use std::collections::BTreeSet;

use either::Either;
use itertools::Itertools;

type Map = BTreeSet<(String, String)>;

#[aoc_generator(day12)]
fn gen(input: &str) -> Map {
    input
        .lines()
        .flat_map(|l| {
            match l.split('-').map(str::to_string).next_tuple() {
                Some((s, e)) => Either::Left([(s.clone(), e.clone()), (e, s)]),
                None => Either::Right([]),
            }
            .into_iter()
        })
        .collect()
}

fn is_small_cave(cave: &str) -> bool {
    cave.bytes()
        .next()
        .map(|b| b.is_ascii_lowercase())
        .expect("node has no name >.<")
}

fn navigate<'a>(
    map: &'a Map,
    current: &'a str,
    visited: BTreeSet<&'a str>,
    path: Vec<&'a str>,
) -> impl Iterator<Item = Vec<&'a str>> {
    if current == "end" {
        Either::Left(std::iter::once(path))
    } else {
        let mut paths = Vec::new();
        for e in map.iter().filter_map(|(s, e)| (s == current).then(|| e)) {
            let mut visited = visited.clone();
            let mut path = path.clone();
            path.push(e);
            if is_small_cave(e) && !visited.insert(e) {
                continue;
            }
            paths.extend(navigate(map, e, visited, path));
        }
        Either::Right(paths.into_iter())
    }
}

#[aoc(day12, part1)]
fn part1(map: &Map) -> usize {
    let start = ["start"];
    navigate(&map, "start", start.into(), start.into()).count()
}

fn navigate_alt<'a>(
    map: &'a Map,
    current: &'a str,
    visited: BTreeSet<&'a str>,
    path: Vec<&'a str>,
    can_visit_small_twice: bool,
) -> impl Iterator<Item = Vec<&'a str>> {
    if current == "end" {
        Either::Left(std::iter::once(path))
    } else {
        let mut paths = Vec::new();
        for e in map.iter().filter_map(|(s, e)| (s == current).then(|| e)) {
            let mut visited = visited.clone();
            let mut path = path.clone();
            path.push(e);
            let mut can_visit_small_twice = can_visit_small_twice;
            if is_small_cave(e) && !visited.insert(e) {
                if !can_visit_small_twice {
                    continue;
                }
                can_visit_small_twice = false;
            }
            paths.extend(navigate_alt(map, e, visited, path, can_visit_small_twice));
        }
        Either::Right(paths.into_iter())
    }
}
#[aoc(day12, part2)]
fn part2(map: &Map) -> usize {
    let start = ["start"];
    navigate_alt(&map, "start", start.into(), start.into(), true).count()
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeSet, HashSet};

    use itertools::Itertools;

    use super::gen;

    const EXAMPLE1: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const PATHS1: &str = r"start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end";

    const PATHS1_part2: &str = r"start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end";

    const EXAMPLE2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn navigate() {
        let map = gen(EXAMPLE1);
        let start = ["start"];
        let computed: HashSet<_> = super::navigate(&map, "start", start.into(), start.into())
            .map(|p| p.join(","))
            .collect();

        let expected: HashSet<_> = PATHS1.lines().map(str::to_string).collect();
        assert_eq!(expected, computed);
    }
    #[test]
    fn navigate_part2() {
        let map = gen(EXAMPLE1);
        let start = ["start"];
        let computed: HashSet<_> =
            super::navigate_alt(&map, "start", start.into(), start.into(), true)
                .map(|p| p.join(","))
                .collect();

        let expected: HashSet<_> = PATHS1_part2.lines().map(str::to_string).collect();
        assert_eq!(expected, computed);
    }

    #[test]
    fn part1() {
        assert_eq!(10, super::part1(&gen(EXAMPLE1)));
        assert_eq!(19, super::part1(&gen(EXAMPLE2)));
        assert_eq!(226, super::part1(&gen(EXAMPLE3)));
    }
    #[test]
    fn part2() {
        assert_eq!(36, super::part2(&gen(EXAMPLE1)));
    }
}
