use aoc_runner_derive::*;

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | if half == 'B' || half == 'R' { 1 } else { 0 }
            })
        })
        .max()
        .expect("input is empty")
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let mut boarding_passes: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | if half == 'B' || half == 'R' { 1 } else { 0 }
            })
        })
        .collect();
    boarding_passes.sort_unstable();

    (boarding_passes[0]..=boarding_passes[boarding_passes.len() - 1])
        .zip(boarding_passes.iter())
        .find(|(expected, seat_id)| expected != *seat_id)
        .expect("input is empty")
        .0
}

#[cfg(test)]
mod test {
    const EXAMPLE: [(&str, usize); 4] = [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];

    #[test]
    fn part1() {
        for &(input, expected) in EXAMPLE.iter() {
            assert_eq!(expected, super::part1(input));
        }
    }
}
