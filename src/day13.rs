use std::cmp::Ordering;

use anyhow::{anyhow, Result, bail};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
};
use yaah::{aoc, aoc_generator};

fn ordered(left: &Item, right: &Item) -> Ordering {
    use Item::*;
    match (left, right) {
        (Integer(l), Integer(r)) => l.cmp(r),
        (List(l), List(r)) => {
            for (left, right) in l.iter().zip(r.iter()) {
                match ordered(left, right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                    Ordering::Greater => return Ordering::Greater,
                }
            }
            l.len().cmp(&r.len())
        }
        (List(_), Integer(r)) => ordered(left, &List(vec![Integer(*r)])),
        (Integer(l), List(_)) => ordered(&List(vec![Integer(*l)]), right),
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Integer(usize),
    List(Vec<Item>),
}
impl std::cmp::Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        ordered(self, other)
    }
}
impl std::cmp::PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Eq for Item {}
impl std::cmp::PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn item(input: &'static str) -> nom::IResult<&str, Item, ()> {
    alt((
        map(
            delimited(tag("["), separated_list0(tag(","), item), tag("]")),
            Item::List,
        ),
        map_res(digit1, |s: &str| s.parse::<usize>().map(Item::Integer)),
    ))(input)
}

#[aoc_generator(day13)]
fn day13(input: &'static str) -> Result<Vec<(Item, Item)>> {
    input
        .split("\n\n")
        .map(|l| {
            match l
                .lines()
                .map(|l| item(l).map(|(_, res)| res))
                .collect_tuple()
                .ok_or(anyhow!("incomplete tuple"))?
            {
                (Ok(left), Ok(right)) => Ok((left, right)),
                (Err(e), _) | (_, Err(e)) => bail!(e),
            }
        })
        .try_collect()
}

#[aoc(day13, part1)]
fn day13_part1(list: &[(Item, Item)]) -> usize {
    list.iter()
        .positions(|(left, right)| left <= right)
        .map(|p| p + 1)
        .sum()
}

#[aoc(day13, part2)]
fn day13_part2(list: &[(Item, Item)]) -> usize {
    use Item::*;
    let start = List(vec![List(vec![Integer(2)])]);
    let stop = List(vec![List(vec![Integer(6)])]);

    let mut list = list
        .iter()
        .flat_map(|(left, right)| [left.clone(), right.clone()].into_iter())
        .collect_vec();
    list.extend_from_slice(&[start.clone(), stop.clone()]);

    list.sort();
    list.iter()
        .positions(|v| {
            let v = v.clone();
            v == start || v == stop
        })
        .map(|p| p + 1)
        .product()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn day13_parser() {
        assert_eq!(8, super::day13(EXAMPLE).unwrap().len());
    }

    #[test]
    fn day13_part1() {
        assert_eq!(13, super::day13_part1(&super::day13(EXAMPLE).unwrap()));
    }

    #[test]
    fn day13_part2() {
        assert_eq!(140, super::day13_part2(&super::day13(EXAMPLE).unwrap()));
    }
}
