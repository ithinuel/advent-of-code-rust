use std::io::BufRead;

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

fn main() {
    let mut pos: (isize, isize) = (0, 0);
    let mut dir = 90;
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .for_each(|inst| {
            let (command, operand) = inst.split_at(1);
            let operand = operand.parse().expect("invalid instruction format");
            match command {
                "F" => move_by(&mut pos, degree_to_cardinal(dir), operand),
                "L" => dir = (dir + 360 - operand) % 360,
                "R" => dir = (dir + operand) % 360,
                d => move_by(&mut pos, d, operand),
            }
        });
    println!("{}", pos.0.abs() + pos.1.abs());
}
