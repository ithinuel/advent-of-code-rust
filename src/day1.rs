use std::io::stdin;
use std::vec::Vec;

fn main() {
    let input = stdin();
    let mut line = String::new();
    if input.read_line(&mut line).is_ok() {
        let line: Vec<u32> = line
            .trim()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        let mut n: u32 =
            line.iter().zip(line.iter().skip(1)).fold(
                0,
                |n, (a, b)| {
                    if a == b {
                        n + a
                    } else {
                        n
                    }
                },
            );
        if line[0] == line[line.len() - 1] {
            n += line[0];
        }
        println!("{}", n);
    }
}
