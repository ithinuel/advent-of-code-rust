use std::collections::HashSet;

fn main() {
    let mut band = HashSet::new();
    let mut cursor = 0isize;
    let mut state = 0;

    for _ in 0..12683008 {
        state = match (state, band.get(&cursor).is_some()) {
            (0, false) => {
                band.insert(cursor);
                cursor += 1;
                1
            }
            (0, true) => {
                band.remove(&cursor);
                cursor -= 1;
                1
            }
            (1, false) => {
                band.insert(cursor);
                cursor -= 1;
                2
            }
            (1, true) => {
                band.remove(&cursor);
                cursor += 1;
                4
            }
            (2, false) => {
                band.insert(cursor);
                cursor += 1;
                4
            }
            (2, true) => {
                band.remove(&cursor);
                cursor -= 1;
                3
            }
            (3, _) => {
                band.insert(cursor);
                cursor -= 1;
                0
            }
            (4, false) => {
                // write 0 on 0 => noop
                cursor += 1;
                0
            }
            (4, true) => {
                band.remove(&cursor);
                cursor += 1;
                5
            }
            (5, false) => {
                band.insert(cursor);
                cursor += 1;
                4
            }
            (5, true) => {
                // write 1 on 1 => noop
                cursor += 1;
                0
            }
            _ => todo!(),
        }
    }
    println!("part1: {}", band.len());
}
