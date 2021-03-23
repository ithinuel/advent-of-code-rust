use aoc_runner_derive::*;

fn move_by(pos: &mut (isize, isize), dir: &str, distance: isize) {
    match dir {
        "N" => pos.1 += distance,
        "S" => pos.1 -= distance,
        "E" => pos.0 += distance,
        "W" => pos.0 -= distance,
        _ => unreachable!(),
    }
}
fn degree_to_cardinal(dir: isize) -> &'static str {
    match dir {
        0 => "N",
        90 => "E",
        180 => "S",
        270 => "W",
        _ => unreachable!(),
    }
}

fn rotate_by(waypoint: &mut (isize, isize), dir: isize) {
    match dir {
        0 => {}
        90 => *waypoint = (waypoint.1, -waypoint.0),
        180 => *waypoint = (-waypoint.0, -waypoint.1),
        270 => *waypoint = (-waypoint.1, waypoint.0),
        _ => unreachable!(),
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> isize {
    let mut pos: (isize, isize) = (0, 0);
    let mut dir = 90;
    input.lines().for_each(|inst| {
        let (command, operand) = inst.split_at(1);
        let operand = operand.parse().expect("invalid instruction format");
        match command {
            "F" => move_by(&mut pos, degree_to_cardinal(dir), operand),
            "L" => dir = (dir + 360 - operand) % 360,
            "R" => dir = (dir + operand) % 360,
            d => move_by(&mut pos, d, operand),
        }
    });
    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> isize {
    let mut pos: (isize, isize) = (0, 0);
    let mut waypoint: (isize, isize) = (10, 1);
    input.lines().for_each(|inst| {
        let (command, operand) = inst.split_at(1);
        let operand = operand.parse().expect("invalid instruction format");
        match command {
            "F" => {
                pos.0 += waypoint.0 * operand;
                pos.1 += waypoint.1 * operand;
            }
            "L" => rotate_by(&mut waypoint, 360 - operand),
            "R" => rotate_by(&mut waypoint, operand),
            d => move_by(&mut waypoint, d, operand),
        }
    });
    pos.0.abs() + pos.1.abs()
}
