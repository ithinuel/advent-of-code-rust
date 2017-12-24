use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lines = input.lock().lines();
    let count:u32 = lines.map(|line| {
        let line = line.unwrap();
        let line: Vec<(&str, Vec<char>)> = line.trim()
            .split(" ")
            .map(|word| {
                let mut aword: Vec<char> = word.chars().collect();
                aword.sort();
                (word, aword)
            })
            .collect();
        let count: u32 = line.iter().enumerate()
            .map(|(i, &(ref word, ref aword))| {
                line.iter()
                    .skip(i+1)
                    .map(|&(ref word2, ref aword2)| {
                        if word == word2 || aword == aword2 {
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
