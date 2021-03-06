use std::io::BufRead;

fn main() {
    let mut step: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.chars().collect())
        .collect();

    fn adjacents<'a>(
        map: &'a Vec<Vec<char>>,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = char> + 'a {
        #[rustfmt::skip]
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];
        DIRECTIONS.iter().filter_map(move |(dx, dy)| {
            (1..)
                .map(|distance| {
                    Some(
                        *map.get(((y as isize) + dy * distance) as usize)?
                            .get(((x as isize) + dx * distance) as usize)?,
                    )
                })
                .filter(|&place| place != Some('.'))
                .next()
                .flatten()
        })
    }

    loop {
        let mut changed = false;
        let new = step
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, seat)| match seat {
                        '.' => '.',
                        'L' => {
                            if adjacents(&step, x, y).all(|place| place == 'L') {
                                changed = true;
                                '#'
                            } else {
                                'L'
                            }
                        }
                        '#' => {
                            if adjacents(&step, x, y).filter(|&place| place == '#').count() >= 5 {
                                changed = true;
                                'L'
                            } else {
                                '#'
                            }
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        if !changed {
            break;
        }
        step = new;
    }

    println!(
        "{}",
        step.iter()
            .map(|line| { line.iter().filter(|&place| place == &'#').count() })
            .sum::<usize>()
    );
}
