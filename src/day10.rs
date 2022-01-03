use std::io::stdin;
use std::vec::Vec;

type Array = Vec<u8>;

fn swap_slice_at(v: &Array, at: usize) -> Array {
    let (start, end) = v.split_at(at);
    let mut new = Vec::with_capacity(v.len());
    new.extend_from_slice(end);
    new.extend_from_slice(start);
    new
}

fn print(v: &Array, at: usize) {
    let v2 = swap_slice_at(v, v.len() - at);
    let p: Vec<String> = v2
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if i == at {
                format!("[{}]", n)
            } else {
                format!("{}", n)
            }
        })
        .collect();
    println!("[{}]", p.join(","));
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let len = input.trim().parse().unwrap();
    let mut v: Array = Array::with_capacity(len);
    for i in 0..len {
        v.push(i as u8);
    }

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let sizes: Vec<usize> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut skip_size: usize = 0;
    let mut cur_pos = 0;
    //print(&v, skip_size);
    for length in sizes {
        v[0..length].reverse();
        let jmp = (length + skip_size) % v.len();
        cur_pos = (cur_pos + jmp) % v.len();
        v = swap_slice_at(&v, jmp);
        //println!("{}.{}", cur_pos, jmp);
        //println!("{}.{}: {:?}", skip_size, length, v);
        //print(&v, cur_pos);
        skip_size += 1;
    }
    print(&v, cur_pos);
    println!("{}", skip_size);
}
