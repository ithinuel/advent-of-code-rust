use aoc_runner_derive::*;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day22)]
fn gen(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .skip(1)
                .filter_map(|l| l.parse().ok())
                .collect_vec()
        })
        .collect_tuple()
        .expect("Bad input format")
}

fn combat(p1: &[usize], p2: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut p1: VecDeque<_> = p1.iter().cloned().collect();
    let mut p2: VecDeque<_> = p2.iter().cloned().collect();

    while let (Some(&c1), Some(&c2)) = (p1.get(0), p2.get(0)) {
        p1.pop_front();
        p2.pop_front();

        if c1 > c2 {
            p1.push_back(usize::max(c1, c2));
            p1.push_back(usize::min(c1, c2));
        } else {
            p2.push_back(usize::max(c1, c2));
            p2.push_back(usize::min(c1, c2));
        }
    }
    (p1.into(), p2.into())
}

#[aoc(day22, part1)]
fn part1((p1, p2): &(Vec<usize>, Vec<usize>)) -> usize {
    let (p1, p2) = combat(p1, p2);

    let iter = if p1.is_empty() { p2.iter() } else { p1.iter() };
    iter.rev()
        .enumerate()
        .map(|(idx, card)| card * (1 + idx))
        .sum::<usize>()
}

enum Player {
    P1(usize),
    P2(usize),
}
impl Player {
    fn from((player, deck): (usize, &VecDeque<usize>)) -> Self {
        let score = deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| card * (1 + idx))
            .sum::<usize>();
        if player == 1 {
            Player::P1(score)
        } else {
            Player::P2(score)
        }
    }
}
/// returns which player won
fn recusive_combat(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> Player {
    let mut previous_rounds = HashSet::new();
    previous_rounds.insert((1, p1.clone()));
    previous_rounds.insert((2, p2.clone()));

    while let (Some(&c1), Some(&c2)) = (p1.get(0), p2.get(0)) {
        p1.pop_front();
        p2.pop_front();

        let winner = if c1 <= p1.len() && c2 <= p2.len() {
            let p1 = p1.range(0..c1).cloned().collect();
            let p2 = p2.range(0..c2).cloned().collect();
            recusive_combat(p1, p2)
        } else if c1 > c2 {
            Player::P1(0)
        } else {
            Player::P2(0)
        };

        if let Player::P1(_) = winner {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }

        if !(previous_rounds.insert((1, p1.clone())) && previous_rounds.insert((2, p2.clone()))) {
            return Player::from((1, &p1));
        }
    }
    if p2.is_empty() {
        Player::from((1, &p1))
    } else {
        Player::from((2, &p2))
    }
}

#[aoc(day22, part2)]
fn part2((p1, p2): &(Vec<usize>, Vec<usize>)) -> usize {
    match recusive_combat(p1.iter().cloned().collect(), p2.iter().cloned().collect()) {
        Player::P1(score) => score,
        Player::P2(score) => score,
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    const INFINITE_EXAMPLE: &str = r"Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn part1() {
        assert_eq!(306, super::part1(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(291, super::part2(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part2_infinite() {
        // this test succeeds if it ever ends :D
        super::part2(&super::gen(INFINITE_EXAMPLE));
    }
}
