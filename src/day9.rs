use std::collections::HashSet;

use anyhow::{anyhow, Error};
use itertools::Itertools;
use yaah::aoc;

fn draw(history: &HashSet<(i32, i32)>, rope: &[(i32, i32)]) {
    let min_maxes = history.iter().fold((0, 0, 0, 0), |(a, b, c, d), &(x, y)| {
        (a.min(x), b.max(x), c.min(y), d.max(y))
    });
    let (min_x, max_x, min_y, max_y) = rope.iter().fold(min_maxes, |(a, b, c, d), &(x, y)| {
        (a.min(x), b.max(x), c.min(y), d.max(y))
    });

    let x_len = max_x - min_x + 1;
    let y_len = max_y - min_y + 1;

    let mut map = vec![b'.'; (x_len * y_len) as usize];
    let mut place_at = |(x, y), c| {
        let coord = x - min_x + (y - min_y) * x_len;
        map[coord as usize] = c;
    };
    history.iter().for_each(|&pos| place_at(pos, b'#'));

    place_at((0, 0), b's');
    rope.iter().enumerate().rev().for_each(|(i, &pos)| {
        let c = if i == (rope.len() - 1) {
            b'T'
        } else if i == 0 {
            b'H'
        } else {
            b'0' + (i as u8)
        };
        place_at(pos, c);
    });

    println!();
    println!("----------------------------");
    println!();

    map.chunks(x_len as usize).rev().for_each(|chunk| {
        println!("{}", String::from_utf8_lossy(chunk));
    });
}

fn update(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    assert!((head.0 - tail.0).abs() <= 2);
    assert!((head.1 - tail.1).abs() <= 2);
    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
        match (head, tail) {
            ((a, b), (c, d)) if a > c && b == d => tail.0 += 1,
            ((a, b), (c, d)) if a < c && b == d => tail.0 -= 1,
            ((a, b), (c, d)) if a == c && b > d => tail.1 += 1,
            ((a, b), (c, d)) if a == c && b < d => tail.1 -= 1,
            ((a, b), (c, d)) if a > c && b > d => {
                tail.0 += 1;
                tail.1 += 1
            }
            ((a, b), (c, d)) if a > c && b < d => {
                tail.0 += 1;
                tail.1 -= 1
            }
            ((a, b), (c, d)) if a < c && b < d => {
                tail.0 -= 1;
                tail.1 -= 1
            }
            ((a, b), (c, d)) if a < c && b > d => {
                tail.0 -= 1;
                tail.1 += 1
            }
            (_, _) => unreachable!(),
        }
    }
    tail
}

fn simulate(
    input: &str,
    rope: &mut [(i32, i32)],
    history: &mut HashSet<(i32, i32)>,
) -> anyhow::Result<()> {
    input.lines().try_for_each(|l| {
        let (dir, steps) = l
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Failed to parse input"))?;
        let steps: usize = steps.parse()?;
        for _ in 0..steps {
            let head = &mut rope[0];
            match dir {
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "L" => head.0 -= 1,
                "D" => head.1 -= 1,
                _ => unreachable!(),
            }
            for i in 1..rope.len() {
                rope[i] = update(rope[i - 1], rope[i]);
            }

            history.insert(*rope.last().unwrap());
        }
        Ok::<(), Error>(())
    })?;
    draw(&history, &rope);
    Ok(())
}

#[aoc(day9, part1)]
fn day9_part1(input: &'static str) -> anyhow::Result<usize> {
    let mut history = HashSet::new();

    simulate(input, &mut [(0, 0), (0, 0)], &mut history)?;

    Ok(history.len())
}

#[aoc(day9, part2)]
fn day9_part2(input: &'static str) -> anyhow::Result<usize> {
    let mut history = HashSet::new();

    simulate(input, &mut [(0i32, 0i32); 10], &mut history)?;

    Ok(history.len())
}

#[cfg(test)]
mod test {
    #[test]
    fn day9_part1() {
        assert_eq!(
            Some(13),
            super::day9_part1(
                r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )
            .ok()
        );
    }
    #[test]
    fn day9_part2() {
        assert_eq!(
            Some(36),
            super::day9_part2(
                r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )
            .ok()
        );
    }
}
