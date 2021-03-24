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

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"939
7,13,x,x,59,x,31,19";

    const EXAMPLE_AS_ARRAY: &[(usize, usize)] = &[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
    const EXAMPLE2_AS_ARRAY: &[(usize, usize)] = &[(0, 17), (2, 13), (3, 19)];
    const EXAMPLE3_AS_ARRAY: &[(usize, usize)] = &[(0, 67), (1, 7), (2, 59), (3, 61)];
    const EXAMPLE4_AS_ARRAY: &[(usize, usize)] = &[(0, 67), (2, 7), (3, 59), (4, 61)];
    const EXAMPLE5_AS_ARRAY: &[(usize, usize)] = &[(0, 67), (1, 7), (3, 59), (4, 61)];
    const EXAMPLE6_AS_ARRAY: &[(usize, usize)] = &[(0, 1789), (1, 37), (2, 47), (3, 1889)];

    #[test]
    fn part1() {
        assert_eq!(59 * 5, super::part1(EXAMPLE));
    }

    #[test]
    fn gen_part2() {
        assert_eq!(
            Some(EXAMPLE_AS_ARRAY),
            super::gen_part2(EXAMPLE).as_ref().map(Vec::as_slice)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(1068781, super::part2(EXAMPLE_AS_ARRAY));
        assert_eq!(3417, super::part2(EXAMPLE2_AS_ARRAY));
        assert_eq!(754018, super::part2(EXAMPLE3_AS_ARRAY));
        assert_eq!(779210, super::part2(EXAMPLE4_AS_ARRAY));
        assert_eq!(1261476, super::part2(EXAMPLE5_AS_ARRAY));
        assert_eq!(1202161486, super::part2(EXAMPLE6_AS_ARRAY));
    }
}
