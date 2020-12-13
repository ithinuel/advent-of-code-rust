use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    let ts: usize = lines
        .next()
        .and_then(|s| s.parse().ok())
        .expect("invalid arrival timestamp");
    let bus_lines: Vec<usize> = lines
        .next()
        .map(|l| l.split(",").filter_map(|s| s.parse().ok()).collect())
        .expect("invalid bus lines format");

    let earliest = bus_lines
        .iter()
        .map(|&id| (id, id - (ts % id)))
        .min_by_key(|&(_, t)| t)
        .expect("could not find a solution");
    println!("{}", earliest.0 * earliest.1)
}
