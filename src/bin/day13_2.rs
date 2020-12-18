use std::io::BufRead;

fn mod_pow(mut n: usize, mut m: usize, modulus: usize) -> usize {
    let mut res = 1;
    n %= modulus;
    while m > 0 {
        if (m & 1) == 1 {
            res = (res * n) % modulus;
        }
        m >>= 1;
        n = (n * n) % modulus;
    }
    res
}

fn main() {
    let bus_lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .skip(1)
        .next()
        .map(|l| {
            l.split(",")
                .enumerate()
                .filter_map(|(n, s)| s.parse().ok().map(|v: usize| (n, v)))
                .collect()
        })
        .expect("invalid bus lines format");

    // Chineese reminder theorem and modular multiplicative inverse
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
    let ts: usize = bus_lines
        .iter()
        .copied()
        .map(|(idx, lane)| -> usize {
            let product: usize = bus_lines
                .iter()
                .map(|&(_, v)| v)
                .filter(|&v| v != lane)
                .product();
            product * idx * mod_pow(product, lane - 2, lane)
        })
        .sum();
    let prods: usize = bus_lines.iter().map(|&(_, v)| v).product();

    println!("{:?}", prods - (ts % prods));
}
