use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<u32> {
    let mut data: Vec<_> = input.lines().filter_map(|s| s.parse().ok()).collect();
    data.sort_unstable();
    data
}

fn look_up(base: u32, data: &[u32]) -> Option<u32> {
    data.iter()
        .find(|&a| {
            if base < *a {
                return false;
            }
            let b = base - a;
            data.binary_search(&b).is_ok()
        })
        .copied()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> Option<u32> {
    look_up(2020, input).map(|a| {
        let b = 2020 - a;
        a * b
    })
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> Option<u32> {
    for a in input {
        let aa = 2020 - a;
        if let Some(b) = look_up(aa, input) {
            let c = aa - b;
            return Some(a * b * c);
        }
    }
    None
}
