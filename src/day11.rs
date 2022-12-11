use itertools::Itertools;
use yaah::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MulOp {
    Imm(usize),
    Old,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectOp {
    Add(usize),
    Mul(MulOp),
}
impl InspectOp {
    fn inspect(&self, v: usize) -> usize {
        match *self {
            InspectOp::Add(imm) => v + imm,
            InspectOp::Mul(MulOp::Imm(imm)) => v * imm,
            InspectOp::Mul(MulOp::Old) => v * v,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    items: Vec<usize>,
    inspect: InspectOp,
    check: (usize, usize, usize),
}

#[aoc_generator(day11)]
fn day11(input: &'static str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|monkey| {
            let mut lines = monkey.lines().skip(1);
            let items = lines
                .next()?
                .split(':')
                .skip(1)
                .flat_map(|v| v.trim().split(", "))
                .filter_map(|v| v.parse().ok())
                .collect_vec();

            let (operand, operator) = lines.next()?.split(' ').rev().take(2).collect_tuple()?;
            let inspect = match (operator, operand) {
                ("+", imm) => InspectOp::Add(imm.parse().ok()?),
                ("*", "old") => InspectOp::Mul(MulOp::Old),
                ("*", imm) => InspectOp::Mul(MulOp::Imm(imm.parse().ok()?)),
                _ => unreachable!(),
            };

            let check = lines
                .filter_map(|l| l.split(' ').last()?.parse().ok())
                .collect_tuple()?;
            Some(Monkey {
                items,
                inspect,
                check,
            })
        })
        .collect()
}

#[aoc(day11, part1)]
fn day11_part1(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut monkey_businesses = vec![0; monkeys.len()];

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[monkey_id].items);
            monkey_businesses[monkey_id] += items.len();

            items.into_iter().for_each(|v| {
                let monkey = &monkeys[monkey_id];
                let mut new = monkey.inspect.inspect(v);
                new /= 3;
                let target = if (new % monkey.check.0) == 0 {
                    monkey.check.1
                } else {
                    monkey.check.2
                };
                monkeys[target].items.push(new);
            });
        }
    }
    monkey_businesses.sort_unstable();
    monkey_businesses.into_iter().rev().take(2).product()
}

#[aoc(day11, part2)]
fn day11_part2(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut monkey_businesses = vec![0; monkeys.len()];

    let master_mod: usize = monkeys.iter().map(|m| m.check.0).product();

    for _round in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[monkey_id].items);
            monkey_businesses[monkey_id] += items.len();

            items.into_iter().for_each(|v| {
                let monkey = &monkeys[monkey_id];
                let mut new = monkey.inspect.inspect(v);
                new %= master_mod;
                let target = if (new % monkey.check.0) == 0 {
                    monkey.check.1
                } else {
                    monkey.check.2
                };
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
    const EXAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    fn example_monkeys() -> Vec<Monkey> {
        use super::InspectOp::*;
        use super::MulOp::*;
        vec![
            Monkey {
                items: vec![79, 98],
                inspect: Mul(Imm(19)),
                check: (23, 2, 3),
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                inspect: Add(6),
                check: (19, 2, 0),
            },
            Monkey {
                items: vec![79, 60, 97],
                inspect: Mul(Old),
                check: (13, 1, 3),
            },
            Monkey {
                items: vec![74],
                inspect: Add(3),
                check: (17, 0, 1),
            },
        ]
    }

    #[test]
    fn day11_gen() {
        assert_eq!(example_monkeys(), super::day11(EXAMPLE));
    }

    #[test]
    fn day11_part1() {
        assert_eq!(10605, super::day11_part1(&example_monkeys()));
    }
    #[test]
    fn day11_part2() {
        assert_eq!(2713310158, super::day11_part2(&example_monkeys()));
    }
}
