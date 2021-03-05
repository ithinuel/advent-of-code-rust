use std::io::BufRead;

const POLICY: &str = r"^(\d+)-(\d+) (\w): (\w+)$";

fn main() {
    let re = regex::Regex::new(POLICY).unwrap();

    let result = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|s| re.is_match(&s))
        .flat_map(|s| {
            re.captures_iter(&s).find_map(|cap| {
                let first: usize = cap.get(1)?.as_str().parse().ok()?;
                let next: usize = cap.get(2)?.as_str().parse().ok()?;
                let c: char = cap.get(3)?.as_str().parse().ok()?;
                let pwd: String = cap.get(4)?.as_str().into();

                Some((first, next, c, pwd))
            }) // we only expect 1 match per line
        })
        .filter(|&(first, next, ch, ref pwd)| {
            let first = pwd.chars().nth(first - 1).map(|c| c == ch).unwrap_or(false);
            let next = pwd.chars().nth(next - 1).map(|c| c == ch).unwrap_or(false);
            first ^ next
        })
        .count();
    println!("{:?}", result);
}
