use std::io::BufRead;

fn check_slope(forest_pattern: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    forest_pattern
        .iter()
        .step_by(slope.0)
        .enumerate()
        .filter(|(column, v)| v[(column * slope.1) % v.len()])
        .count()
}

fn main() {
    let forest_pattern: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.chars().map(|c| c == '#').collect())
        .filter(|l: &Vec<bool>| !l.is_empty())
        .collect();

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let result: usize = slopes
        .iter()
        .map(|&slope| check_slope(&forest_pattern, slope))
        .product();
    println!("{:?}", result);
}
