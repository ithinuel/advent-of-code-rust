use std::io::stdin;
use std::io::BufRead;

fn look_up(base: u32, data: &Vec<u32>) -> Option<u32> {
    data.iter()
        .find(|&a| {
            if base < *a {
                return false;
            }
            let b = base - a;
            data.binary_search(&b).is_ok()
        })
        .copied()
}

fn main() {
    let input = stdin();
    let mut data: Vec<u32> = input
        .lock()
        .lines()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse().ok())
        .collect();

    data.sort();
    if let Some(a) = look_up(2020, &data) {
        let b = 2020 - a;
        println!("{}*{}={}", a, b, a * b);
    } else {
        println!("nomatch");
    }

    for a in &data {
        let aa = 2020 - a;
        if let Some(b) = look_up(aa, &data) {
            let c = aa - b;
            println!("{}*{}*{}={}", a, b, c, a * b * c);
            break;
        }
    }
}
