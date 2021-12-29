use aoc_runner_derive::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|line| {
            let (inst, op) = line.split_at(4);
            let op = op.parse().expect("Invalid signed integer");
            match &inst[..3] {
                "acc" => Inst::Acc(op),
                "jmp" => Inst::Jmp(op),
                "nop" => Inst::Nop(op),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(program: &[Inst]) -> i32 {
    let mut visited: Vec<_> = [false].repeat(program.len());
    let mut pc = 0;
    let mut acc = 0;
    while pc < program.len() && !visited[pc] {
        visited[pc] = true;
        match program[pc] {
            Inst::Nop(_) => pc += 1,
            Inst::Acc(op) => {
                pc += 1;
                acc += op
            }
            Inst::Jmp(op) => pc = ((pc as i32) + op) as usize,
        }
    }
    acc
}

fn run_without_loop(program: &[Inst]) -> Option<i32> {
    let mut visited: Vec<_> = [false].repeat(program.len());
    let mut pc = 0;
    let mut acc = 0;
    while pc < program.len() && !visited[pc] {
        visited[pc] = true;
        match program[pc] {
            Inst::Nop(_) => pc += 1,
            Inst::Acc(op) => {
                pc += 1;
                acc += op
            }
            Inst::Jmp(op) => pc = ((pc as i32) + op) as usize,
        }
    }

    if pc == program.len() {
        Some(acc)
    } else {
        None
    }
}

#[aoc(day8, part2)]
fn part2(program: &[Inst]) -> anyhow::Result<i32> {
    let mut program = program.to_vec();
    for ptr in 0..program.len() {
        // find nop or jmp and flip it
        program[ptr] = match program[ptr] {
            Inst::Acc(_) => continue,
            Inst::Nop(op) => Inst::Jmp(op),
            Inst::Jmp(op) => Inst::Nop(op),
        };

        if let Some(acc) = run_without_loop(&program) {
            return Ok(acc);
        }
        // restore nop or jmp
        program[ptr] = match program[ptr] {
            Inst::Acc(_) => unreachable!(),
            Inst::Nop(op) => Inst::Jmp(op),
            Inst::Jmp(op) => Inst::Nop(op),
        };
    }
    anyhow::bail!("No solution found");
}

#[cfg(test)]
mod test {
    use super::Inst;
    const EXAMPLE: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const EXAMPLE_AS_ARRAY: &[Inst] = &[
        Inst::Nop(0),
        Inst::Acc(1),
        Inst::Jmp(4),
        Inst::Acc(3),
        Inst::Jmp(-3),
        Inst::Acc(-99),
        Inst::Acc(1),
        Inst::Jmp(-4),
        Inst::Acc(6),
    ];

    #[test]
    fn generator() {
        assert_eq!(EXAMPLE_AS_ARRAY, super::generator(EXAMPLE));
    }

    #[test]
    fn part1() {
        assert_eq!(5, super::part1(EXAMPLE_AS_ARRAY));
    }

    #[test]
    fn part2() {
        assert_eq!(Some(8), super::part2(EXAMPLE_AS_ARRAY).ok());
    }
}
