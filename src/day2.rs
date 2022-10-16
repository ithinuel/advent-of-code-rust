use yaah::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .filter_map(|l| {
            let mut tokens = l.split_whitespace();
            let cmd = tokens.next()?;
            let val = tokens.next()?.parse().ok()?;
            Some((cmd.to_string(), val))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(String, i32)]) -> i32 {
    let (mut h, mut d) = (0, 0);

    for (cmd, val) in input {
        match cmd.as_str() {
            "forward" => h += val,
            "up" => d -= val,
            "down" => d += val,
            _ => unreachable!(),
        }
    }
    h * d
}

#[aoc(day2, part2)]
fn part2(input: &[(String, i32)]) -> i32 {
    let (mut h, mut d, mut aim) = (0, 0, 0);

    for (cmd, val) in input {
        match cmd.as_str() {
            "forward" => {
                h += val;
                d += val * aim;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => unreachable!(),
        }
    }
    h * d
}

#[cfg(test)]
mod test {
    use super::{gen, part1 as solve_part1, part2 as solve_part2};

    const EXAMPLE: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1() {
        assert_eq!(150, solve_part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(900, solve_part2(&gen(EXAMPLE)));
    }
}
