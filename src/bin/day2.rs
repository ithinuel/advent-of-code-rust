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
            re.captures_iter(&s)
                .filter_map(|cap| {
                    let min: usize = cap.get(1)?.as_str().parse().ok()?;
                    let max = cap.get(2)?.as_str().parse().ok()?;
                    let c: char = cap.get(3)?.as_str().parse().ok()?;
                    let pwd: String = cap.get(4)?.as_str().into();
                    Some((min..=max, c, pwd))
                })
                .next() // we only expect 1 match per line
        })
        .filter(|(range, ch, pwd)| {
            let occurences = pwd.chars().filter(|c| c == ch).count();
            range.contains(&occurences)
        })
        .count();
    println!("{:?}", result);
}
