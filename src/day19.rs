use std::io::stdin;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = stdin();
    let map: Vec<Vec<char>> = input
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut x = map[0].iter().position(|c| c == &'|').unwrap();
    let mut y = 0;
    let mut dir = Direction::Down;

    y += 1;

    let mut history = String::new();

    loop {
        //println!("({},{},{:?})", x, y, dir);
        let c = &map[y][x];
        match c {
            &'|' => {}
            &'-' => {}
            &'+' => {
                dir = match dir {
                    Direction::Up | Direction::Down => {
                        if map[y][x + 1] != ' ' {
                            Direction::Right
                        } else if map[y][x - 1] != ' ' {
                            Direction::Left
                        } else {
                            panic!("can't go right or left");
                        }
                    }
                    Direction::Left | Direction::Right => {
                        if map[y - 1][x] != ' ' {
                            Direction::Up
                        } else if map[y + 1][x] != ' ' {
                            Direction::Down
                        } else {
                            panic!("cannot go up or down");
                        }
                    }
                }
            }
            &' ' => {
                break;
            }
            a => {
                history.push(*a);
            }
        }
        match dir {
            Direction::Up if y != 0 => y -= 1,
            Direction::Down if y < map.len() => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            _ => panic!(
                "hmmm ?? (x,y,dir)=({},{},{:?}) for a map ({},{})",
                x,
                y,
                dir,
                map[0].len(),
                map.len()
            ),
        }
    }
    println!("{}", history);
}
