use std::io::BufRead;

fn main() {
    let (preamble_len, data): (usize, Vec<usize>) = {
        let input = std::io::stdin();
        let mut input = input.lock().lines().filter_map(Result::ok);
        let preamble_len = input
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Invalid preamble length");
        let data = input.filter_map(|s| s.parse().ok()).collect();
        (preamble_len, data)
    };

    let part1 = data
        .iter()
        .enumerate()
        .skip(preamble_len)
        .find(|&(i, &n)| {
            let allowed_values = &data[i - preamble_len..i];
            !allowed_values.iter().any(|&v| {
                n.checked_sub(v)
                    .map(|r| allowed_values.contains(&r))
                    .unwrap_or(false)
            })
        })
        .map(|(_, &v)| v)
        .expect("Unable to find the invalid number");

    println!("{}", part1);
}
