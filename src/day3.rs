use aoc_helper::*;

fn solve_part1<const N: usize>(input: &str) -> usize {
    let mut ones_cnt = [0; N];
    let mut lines_cnt = 0;
    for input in input.lines() {
        lines_cnt += 1;
        for (i, b) in input.bytes().enumerate() {
            if b == b'1' {
                ones_cnt[i] += 1;
            }
        }
    }

    let mask = (1 << N) - 1;
    let gamma = ones_cnt.iter().fold(0, |acc, &n| {
        acc << 1 | if n > (lines_cnt - n) { 1 } else { 0 }
    });

    gamma * (!gamma & mask)
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    solve_part1::<12>(input)
}

fn count_pop_for_bit(inputs: &[String], bit: usize) -> usize {
    let mut count = 0;
    for input in inputs {
        let b = input.as_bytes()[bit];
        if b == b'1' {
            count += 1;
        }
    }
    count
}
enum Param {
    Oxi,
    Co2,
}
fn find_param<const N: usize>(mut input: Vec<String>, param: Param) -> usize {
    let mut bit = 0..N;
    while let (Some(i), true) = (bit.next(), input.len() > 1) {
        let ones_cnt = count_pop_for_bit(&input, i);
        let zeros_cnt = input.len() - ones_cnt;

        let keep_high = match param {
            Param::Oxi => ones_cnt >= zeros_cnt,
            Param::Co2 => ones_cnt < zeros_cnt,
        };

        input = input
            .into_iter()
            .filter(|n| keep_high ^ (n.as_bytes()[i] == b'1'))
            .collect();
    }
    assert_eq!(1, input.len());
    input[0]
        .as_bytes()
        .iter()
        .fold(0, |acc, &n| acc << 1 | if n == b'1' { 1 } else { 0 })
}

fn solve_part2<const N: usize>(input: &str) -> usize {
    let input: Vec<_> = input.lines().map(str::to_string).collect();
    let oxi = find_param::<N>(input.clone(), Param::Oxi);
    let co2 = find_param::<N>(input, Param::Co2);

    oxi * co2
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    solve_part2::<12>(input)
}

#[cfg(test)]
mod test {
    use super::{solve_part1, solve_part2};

    const EXAMPLE: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1() {
        assert_eq!(198, solve_part1::<5>(&EXAMPLE));
    }

    #[test]
    fn part2() {
        assert_eq!(230, solve_part2::<5>(&EXAMPLE));
    }
}
