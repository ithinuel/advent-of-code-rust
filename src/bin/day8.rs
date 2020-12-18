use std::io::BufRead;

#[derive(Debug)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn main() {
    let program: Vec<_> = std::io::stdin()
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
    println!("{}", acc);
}
