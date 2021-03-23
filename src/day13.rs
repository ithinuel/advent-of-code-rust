use aoc_runner_derive::*;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let ts: usize = lines
        .next()
        .and_then(|s| s.parse().ok())
        .expect("invalid arrival timestamp");
    let bus_lines: Vec<usize> = lines
        .next()
        .map(|l| l.split(',').filter_map(|s| s.parse().ok()).collect())
        .expect("invalid bus lines format");

    let earliest = bus_lines
        .iter()
        .map(|&id| (id, id - (ts % id)))
        .min_by_key(|&(_, t)| t)
        .expect("could not find a solution");
    earliest.0 * earliest.1
}

#[aoc_generator(day13, part2)]
fn gen_part2(input: &str) -> Option<Vec<(usize, usize)>> {
    let res = input
        .lines()
        .nth(1)?
        .split(',')
        .enumerate()
        .filter_map(|(n, s)| s.parse().ok().map(|v: usize| (n, v)))
        .collect();
    Some(res)
}

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

#[aoc(day13, part2)]
fn part2(bus_lines: &[(usize, usize)]) -> usize {
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

    prods - (ts % prods)
}
