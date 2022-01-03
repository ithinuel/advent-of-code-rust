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
        let mut double = line.clone();
        double.extend_from_slice(&line);
        let n: u32 = line
            .iter()
            .zip(double.iter().skip(line.len() / 2))
            .fold(0, |n, (a, b)| if a == b { n + a } else { n });
        println!("{}", n);
    }
}
