use std::io::stdin;
use std::io::Read;

#[derive(Debug)]
struct State {
    depth: u32,
    score: u32,
    garbage: bool,
    escaped: bool
}

fn main() {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    let result = input.chars().fold(State {
        depth: 0,
        score: 0,
        garbage: false,
        escaped: false
    }, |mut state, c| {
        if state.garbage {
            if state.escaped {
                state.escaped = false;
            } else {
                match c {
                    '!' => state.escaped = true,
                    '>' => state.garbage = false,
                    _ => {}
                }
            }
        } else {
            match c {
                '<' => state.garbage = true,
                '{' => {
                    state.depth += 1;
                    state.score += state.depth;
                }
                '}' => state.depth -= 1,
                _ => {}
            }
        }
        state
    });
    println!("{:?}", result);
}
