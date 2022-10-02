use aoc_helper::*;
use itertools::Itertools;
use rayon::prelude::*;

#[aoc_generator(day21)]
fn gen(input: &str) -> (usize, usize) {
    input
        .lines()
        .flat_map(|l| l.split(": ").skip(1))
        .filter_map(|v| v.parse::<usize>().ok())
        .collect_tuple()
        .expect("Invalid input")
}

#[aoc(day21, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let dice = (1..=100)
        .cycle()
        .batching(|it| Some(it.take(3).sum::<usize>()));

    dice.enumerate()
        .try_fold(
            (0, 0, input.0, input.1),
            |(mut p1_score, mut p2_score, mut p1_pos, mut p2_pos), (turn, dice)| {
                let (score, pos) = if turn % 2 == 0 {
                    (&mut p1_score, &mut p1_pos)
                } else {
                    (&mut p2_score, &mut p2_pos)
                };

                *pos = ((*pos - 1) + dice) % 10 + 1;
                *score += *pos;

                if *score >= 1000 {
                    Err(usize::min(p1_score, p2_score) * (turn + 1) * 3)
                } else {
                    Ok((p1_score, p2_score, p1_pos, p2_pos))
                }
            },
        )
        .unwrap_err()
}

//let dice = [1..=3, 1..=3, 1..=3]
//    .into_iter()
//    .multi_cartesian_product()
//    .map(|v| v.into_iter().sum::<usize>())
//    .counts_by(|v| v)
//    .into_iter()
//    .sorted_by_key(|&(_, freq)| freq)
//    .collect_vec();
const DICE: [(usize, usize); 7] = [(3, 1), (9, 1), (4, 3), (8, 3), (7, 6), (5, 6), (6, 7)];

// (position, score)
type State = (usize, usize);
fn play_turn(turn: usize, active: State, other: State) -> (usize, usize) {
    let process = move |(dice, freq)| {
        // compute current player's score
        let pos = (((active.0 - 1) + dice) % 10) + 1;
        let score = active.1 + pos;

        // if current player won account for victory.
        let victories = if score >= 21 {
            (1, 0)
        } else {
            // if not then play another turn
            let (other, active) = play_turn(turn + 1, other, (pos, score));
            (active, other)
        };
        // scale by this universe's frequency.
        (victories.0 * freq, victories.1 * freq)
    };
    if turn < 4 {
        DICE.into_par_iter()
            .map(process)
            .reduce(|| (0, 0), |acc, vic| (acc.0 + vic.0, acc.1 + vic.1))
    } else {
        DICE.into_iter()
            .map(process)
            .fold((0, 0), |acc, vic| (acc.0 + vic.0, acc.1 + vic.1))
    }
}

#[aoc(day21, part2)]
fn part2(input: &(usize, usize)) -> usize {
    let res = play_turn(0, (input.0, 0), (input.1, 0));
    usize::max(res.0, res.1)
}

#[cached::proc_macro::cached]
fn play_turn_cached(active: State, other: State) -> (usize, usize) {
    DICE.into_iter()
        .map(move |(dice, freq)| {
            // compute current player's score
            let pos = (((active.0 - 1) + dice) % 10) + 1;
            let score = active.1 + pos;

            // if current player won account for victory.
            let victories = if score >= 21 {
                (1, 0)
            } else {
                // if not then play another turn
                let (other, active) = play_turn_cached(other, (pos, score));
                (active, other)
            };
            // scale by this universe's frequency.
            (victories.0 * freq, victories.1 * freq)
        })
        .fold((0, 0), |acc, vic| (acc.0 + vic.0, acc.1 + vic.1))
}
#[aoc(day21, part2, cached)]
fn part2_cached(input: &(usize, usize)) -> usize {
    let res = play_turn_cached((input.0, 0), (input.1, 0));
    usize::max(res.0, res.1)
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part1() {
        assert_eq!(739785, super::part1(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(444_356_092_776_315, super::part2(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part2_cached() {
        assert_eq!(
            444_356_092_776_315,
            super::part2_cached(&super::gen(EXAMPLE))
        );
    }
}
