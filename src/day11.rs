use yaah::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Monkey {
    items: Vec<usize>,
    inspect: fn(usize) -> usize,
    check: fn(usize) -> usize,
}

#[aoc_generator(day11)]
fn day11(_: &'static str) -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![62, 92, 50, 63, 62, 93, 73, 50],
            inspect: |v| (v * 7) % 9699690,
            check: |v| if (v % 2) == 0 { 7 } else { 1 },
        },
        Monkey {
            items: vec![51, 97, 74, 84, 99],
            inspect: |v| v + 3,
            check: |v| if (v % 7) == 0 { 2 } else { 4 },
        },
        Monkey {
            items: vec![98, 86, 62, 76, 51, 81, 95],
            inspect: |v| v + 4,
            check: |v| if (v % 13) == 0 { 5 } else { 4 },
        },
        Monkey {
            items: vec![53, 95, 50, 85, 83, 72],
            inspect: |v| v + 5,
            check: |v| if (v % 19) == 0 { 6 } else { 0 },
        },
        Monkey {
            items: vec![59, 60, 63, 71],
            inspect: |v| (v * 5) % 9699690,
            check: |v| if (v % 11) == 0 { 5 } else { 3 },
        },
        Monkey {
            items: vec![92, 65],
            inspect: |v| (v * v) % 9699690,
            check: |v| if (v % 5) == 0 { 6 } else { 3 },
        },
        Monkey {
            items: vec![78],
            inspect: |v| v + 8,
            check: |v| if (v % 3) == 0 { 0 } else { 7 },
        },
        Monkey {
            items: vec![84, 93, 54],
            inspect: |v| v + 1,
            check: |v| if (v % 17) == 0 { 2 } else { 1 },
        },
    ]
}

#[aoc(day11, part1)]
fn day11_part1(monkeys: &Vec<Monkey>) -> usize {
    let mut monkeys = monkeys.clone();
    let mut monkey_businesses = vec![0; monkeys.len()];

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[monkey_id].items, Vec::new());
            monkey_businesses[monkey_id] += items.len();

            items.into_iter().for_each(|v| {
                let monkey = &monkeys[monkey_id];
                let mut new = (monkey.inspect)(v);
                new /= 3;
                let target = (monkey.check)(new);
                monkeys[target].items.push(new);
            });
        }
    }
    monkey_businesses.sort_unstable();
    monkey_businesses.into_iter().rev().take(2).product()
}

#[aoc(day11, part2)]
fn day11_part2(monkeys: &Vec<Monkey>) -> usize {
    let mut monkeys = monkeys.clone();
    let mut monkey_businesses = vec![0; monkeys.len()];

    for _round in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[monkey_id].items, Vec::new());
            monkey_businesses[monkey_id] += items.len();

            items.into_iter().for_each(|v| {
                let monkey = &monkeys[monkey_id];
                let new = (monkey.inspect)(v);
                let target = (monkey.check)(new);
                monkeys[target].items.push(new);
            });
        }
    }
    monkey_businesses.sort_unstable();
    monkey_businesses.into_iter().rev().take(2).product()
}

#[cfg(test)]
mod test {
    use crate::day11::Monkey;
    fn input() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98],
                inspect: |v| (v * 19) % 96577,
                check: |v| if (v % 23) == 0 { 2 } else { 3 },
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                inspect: |v| v + 6,
                check: |v| if (v % 19) == 0 { 2 } else { 0 },
            },
            Monkey {
                items: vec![79, 60, 97],
                inspect: |v| (v * v) % 96577,
                check: |v| if (v % 13) == 0 { 1 } else { 3 },
            },
            Monkey {
                items: vec![74],
                inspect: |v| v + 3,
                check: |v| if (v % 17) == 0 { 0 } else { 1 },
            },
        ]
    }

    #[test]
    fn day11_part1() {
        assert_eq!(10605, super::day11_part1(&input()));
    }
    #[test]
    fn day11_part2() {
        assert_eq!(2713310158, super::day11_part2(&input()));
    }
}
