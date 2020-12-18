use std::io::BufRead;

fn main() {
    let mut boarding_passes: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | if half == 'B' || half == 'R' { 1 } else { 0 }
            })
        })
        .collect();
    boarding_passes.sort();

    let result = (boarding_passes[0]..=boarding_passes[boarding_passes.len() - 1])
        .zip(boarding_passes.iter())
        .find(|(expected, sead_id)| expected != *sead_id)
        .expect("input is not empty");

    println!("{:?}", result.0);
}
