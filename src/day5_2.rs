use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let mut program: Vec<i32> = input.lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
   
    let mut steps = 0;
    let mut program_counter = 0;
    //println!("{:?}", program);
    //println!("-------");
    while program_counter < program.len() {
        let n = program[program_counter];
        if n >= 3 {
            program[program_counter] -= 1;
        } else {
            program[program_counter] += 1;
        }
        program_counter = ((program_counter as i32) + n) as usize;
        //println!("{:?}", program);
        steps += 1;
    }
    println!("{}", steps);
}
