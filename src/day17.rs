use std::ops::RangeInclusive;

use aoc_runner_derive::*;
use itertools::Itertools;

type Input = (RangeInclusive<u32>, RangeInclusive<i32>);
type Vector = (u32, i32);

#[aoc_generator(day17)]
fn gen(input: &str) -> Option<Input> {
    let (x, y) = input
        .trim_start_matches("target area: ")
        .split(", ")
        .next_tuple()?;
    let (x_start, x_end) = x
        .trim_start_matches("x=")
        .split("..")
        .filter_map(|s| s.parse().ok())
        .next_tuple()?;
    let (y_start, y_end) = y
        .trim_start_matches("y=")
        .split("..")
        .filter_map(|s| s.parse().ok())
        .next_tuple()?;
    Some((x_start..=x_end, y_start..=y_end))
}

fn step((pos, speed): (Vector, Vector)) -> (Vector, Vector) {
    (
        (pos.0 + speed.0, pos.1 + speed.1),
        (speed.0.saturating_sub(1), speed.1 - 1),
    )
}

fn shoot(mut speed: Vector, target: &Input) -> Result<(usize, i32), (Vector, Vector)> {
    let mut pos = (0, 0);
    let mut max_y = 0;

    let mut step_cnt = 0;
    let res = loop {
        step_cnt += 1;

        let (new_p, new_s) = step((pos, speed));
        max_y = i32::max(max_y, new_p.1);

        if new_p.0 > *target.0.end() || new_p.1 < *target.1.start() {
            break false;
        } else if target.0.contains(&new_p.0) && target.1.contains(&new_p.1) {
            break true;
        }

        pos = new_p;
        speed = new_s;
    };
    if res {
        Ok((step_cnt, max_y))
    } else {
        Err((pos, speed))
    }
}

#[aoc(day17, part1)]
fn part1(target: &Input) -> i32 {
    let mut speed = (0, 1000);

    loop {
        match shoot(speed, target) {
            Ok((_steps, max_y)) => {
                break max_y;
            }
            Err((pos, _end_speed)) => {
                if pos.0 > *target.0.end() {
                    speed.0 -= 1;
                } else if pos.0 < *target.0.start() {
                    speed.0 += 1;
                } else if pos.1 < *target.1.start() {
                    speed.1 += 1;
                } else {
                    speed.1 -= 1;
                }
            }
        }
    }
}

#[aoc(day17, part2)]
fn part2(target: &Input) -> usize {
    let possible_dx: Vec<_> = (1..=*target.0.end())
        .cartesian_product(0..1000)
        .filter_map(|(dx, step)| {
            let (end_speed, distance) = (0..step).fold((dx, 0), move |(dx, pos), _| {
                (dx.saturating_sub(1), pos + dx)
            });
            target
                .0
                .contains(&distance)
                .then(|| (dx, step, end_speed, distance))
        })
        .collect();

    let max_step = possible_dx.iter().max_by_key(|v| v.1).unwrap().1;
    let possible_dy: Vec<_> = (*target.1.start()..1000)
        .cartesian_product(1..=max_step)
        .map(|(dy, step)| {
            let (end_speed, depth) =
                (0..step).fold((dy, 0), move |(dy, pos), _| (dy - 1, pos + dy));

            (dy, step, end_speed, depth)
        })
        .filter(|(_, _, _, depth)| target.1.contains(depth))
        .collect();

    possible_dx
        .into_iter()
        .cartesian_product(possible_dy.into_iter())
        .filter(|((_, step_x, _, _), (_, step_y, _, _))| step_x == step_y)
        .map(|(x, y)| (x.0, y.0))
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"target area: x=20..30, y=-10..-5";

    #[test]
    fn part1() {
        assert_eq!(Some(45), gen(EXAMPLE).as_ref().map(super::part1));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)]
    fn part2() {
        assert_eq!(Some(112), gen(EXAMPLE).as_ref().map(super::part2));
    }
}
