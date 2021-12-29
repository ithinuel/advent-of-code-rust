use anyhow::{anyhow, Result};
use aoc_runner_derive::*;

#[aoc_generator(day9, part1)]
fn gen_part1(input: &str) -> Result<(usize, Vec<usize>)> {
    let mut input = input.lines();
    let preamble_len = input
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow!("Invalid preamble length"))?;
    let data = input.filter_map(|s| s.parse().ok()).collect();
    Ok((preamble_len, data))
}

#[aoc(day9, part1)]
fn part1((preamble_len, data): &(usize, Vec<usize>)) -> Result<usize> {
    data.iter()
        .enumerate()
        .skip(*preamble_len)
        .find(|&(i, &n)| {
            let allowed_values = &data[i - preamble_len..i];
            !allowed_values.iter().any(|&v| {
                n.checked_sub(v)
                    .map(|r| allowed_values.contains(&r))
                    .unwrap_or(false)
            })
        })
        .map(|(_, &v)| v)
        .ok_or_else(|| anyhow!("Unable to find the invalid number"))
}

#[aoc_generator(day9, part2)]
fn gen_part2(input: &str) -> Result<(usize, Vec<usize>)> {
    let part1_input = gen_part1(input)?;
    let part1_result = part1(&part1_input)?;
    Ok((part1_result, part1_input.1))
}

#[aoc(day9, part2)]
fn part2(&(part1, ref data): &(usize, Vec<usize>)) -> Result<usize> {
    (0..data.len())
        .into_iter()
        .find_map(|base| {
            let end = data
                .iter()
                .enumerate()
                .skip(base)
                .try_fold(0, |acc, (i, &v)| {
                    let sum = acc + v;
                    if sum < part1 {
                        Ok(sum)
                    } else if sum != part1 {
                        Err(None)
                    } else {
                        Err(Some(i))
                    }
                })
                .err()??;
            let (min, max) = &data[base..end]
                .iter()
                .fold((usize::max_value(), 0), |(min, max), &v| {
                    (usize::min(min, v), usize::max(max, v))
                });
            Some(min + max)
        })
        .ok_or_else(|| anyhow!("Unable to find the encryption weakness"))
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    const EXAMPLE: &str = r"5
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    const EXAMPLE_AS_ARRAY: &[usize] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    lazy_static! {
        static ref EXAMPLE_AS_PART1: (usize, (usize, Vec<usize>)) =
            (127, (5, EXAMPLE_AS_ARRAY.to_vec()));
        static ref EXAMPLE_AS_PART2: (usize, (usize, Vec<usize>)) =
            (62, (127, EXAMPLE_AS_ARRAY.to_vec()));
    }

    #[test]
    fn gen_part1() {
        assert_eq!(
            Some(&EXAMPLE_AS_PART1.1),
            super::gen_part1(EXAMPLE).ok().as_ref()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            Some(EXAMPLE_AS_PART1.0),
            super::part1(&EXAMPLE_AS_PART1.1).ok()
        );
    }

    #[test]
    fn gen_part2() {
        assert_eq!(
            Some(&EXAMPLE_AS_PART2.1),
            super::gen_part2(EXAMPLE).ok().as_ref()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Some(EXAMPLE_AS_PART2.0),
            super::part2(&EXAMPLE_AS_PART2.1).ok()
        );
    }
}
