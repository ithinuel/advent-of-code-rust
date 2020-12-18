use std::io::BufRead;

fn main() {
    let forest_pattern: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.chars().map(|c| c == '#').collect())
        .filter(|l: &Vec<bool>| !l.is_empty())
        .collect();

    let result = forest_pattern
        .iter()
        .enumerate()
        .filter(|(n, line)| line[(n * 3 % line.len())])
        .count();
    println!("{:?}", result);
}
