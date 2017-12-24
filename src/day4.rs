use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lines = input.lock().lines();
    let count:u32 = lines.map(|line| {
        let line = line.unwrap();
        let line: Vec<&str> = line.trim().split(" ").collect();
        let count: u32 = line.iter().enumerate()
            .map(|(i, word)| {
                line.iter()
                    .skip(i+1)
                    .map(|word2| {
                        if word == word2 {
                            1
                        } else {
                            0
                        }
                    }).sum::<u32>()
            }).sum();
        count == 0
    }).map(|b| if b {1} else {0})
    .sum();
    println!("{}", count);
}
