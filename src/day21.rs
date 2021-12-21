use aoc_runner_derive::*;
use itertools::Itertools;

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

// (position, score)
type State = (usize, usize);
fn play_turn(turn: usize, p1: State, p2: State) -> (usize, usize) {
    //let dice = [1..=3, 1..=3, 1..=3]
    //    .into_iter()
    //    .multi_cartesian_product()
    //    .map(|v| v.into_iter().sum::<usize>())
    //    .counts_by(|v| v)
    //    .into_iter()
    //    .sorted_by_key(|&(_, freq)| freq)
    //    .collect_vec();
    let dice = [(3, 1), (9, 1), (4, 3), (8, 3), (7, 6), (5, 6), (6, 7)];

    let is_p1 = turn % 2 == 0;
    let (player, other) = if is_p1 { (p1, p2) } else { (p2, p1) };

    dice.into_iter()
        .map(move |(dice, freq)| {
            // compute current player's score
            let pos = (((player.0 - 1) + dice) % 10) + 1;
            // shadowing the `player` variable is convenient!
            let player = (pos, player.1 + pos);

            // if current player won account for victory.
            let (p1v, p2v) = if player.1 >= 21 {
                if is_p1 {
                    (1, 0)
                } else {
                    (0, 1)
                }
            } else {
                // if not then play another turn
                let (np1, np2) = if is_p1 {
                    (player, other)
                } else {
                    (other, player)
                };

                // scale the victories of each player by the frequency of that universe.
                play_turn(turn + 1, np1, np2)
            };
            (p1v * freq, p2v * freq)
        })
        .fold((0, 0), |acc, vic| (acc.0 + vic.0, acc.1 + vic.1))
}

#[aoc(day21, part2)]
fn part2(input: &(usize, usize)) -> usize {
    let res = play_turn(0, (input.0, 0), (input.1, 0));
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
}
