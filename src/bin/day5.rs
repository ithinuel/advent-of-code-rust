use std::io::BufRead;

fn main() {
    let result = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | if half == 'B' || half == 'R' { 1 } else { 0 }
            })
        })
        .max()
        .expect("input is not empty");
    println!("{}", result);
}
