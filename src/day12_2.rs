use std::io::BufRead;

fn move_by(waypoint: &mut (isize, isize), dir: &str, distance: isize) {
    match dir {
        "N" => waypoint.1 += distance,
        "S" => waypoint.1 -= distance,
        "E" => waypoint.0 += distance,
        "W" => waypoint.0 -= distance,
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

fn main() {
    let mut pos: (isize, isize) = (0, 0);
    let mut waypoint: (isize, isize) = (10, 1);
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .for_each(|inst| {
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
    println!("{}", pos.0.abs() + pos.1.abs());
}
