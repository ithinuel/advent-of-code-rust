use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn run_without_loop(program: &Vec<Inst>) -> Option<i32> {
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

fn main() {
    let mut program: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
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
        .collect();

    for ptr in 0..program.len() {
        // find nop or jmp and flip it
        program[ptr] = match program[ptr] {
            Inst::Acc(_) => continue,
            Inst::Nop(op) => Inst::Jmp(op),
            Inst::Jmp(op) => Inst::Nop(op),
        };

        if let Some(acc) = run_without_loop(&program) {
            println!("{}", acc);
            break;
        }
        // restore nop or jmp
        program[ptr] = match program[ptr] {
            Inst::Acc(_) => unreachable!(),
            Inst::Nop(op) => Inst::Jmp(op),
            Inst::Jmp(op) => Inst::Nop(op),
        };
    }
}
