use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

fn play_n_round(starting_values: &[usize]) -> usize {
    let mut map = BTreeMap::new();
    starting_values
        .iter()
        .copied()
        .enumerate()
        .for_each(|(idx, v)| {
            map.insert(v, (None, idx + 1));
        });
    (starting_values.len() + 1..30_000_001).fold(
        *starting_values.last().unwrap(),
        |previous, round| {
            let new = match map.entry(previous) {
                Entry::Vacant(_) => 0,
                Entry::Occupied(entry) => match *entry.get() {
                    (None, _) => 0,
                    (Some(one_before_last), last_seen) => last_seen - one_before_last,
                },
            };
            match map.entry(new) {
                Entry::Vacant(entry) => {
                    entry.insert((None, round));
                }
                Entry::Occupied(mut entry) => match *entry.get() {
                    (None, last_seen) => {
                        entry.insert((Some(last_seen), round));
                    }
                    (Some(_), last_seen) => {
                        entry.insert((Some(last_seen), round));
                    }
                },
            }
            new
        },
    )
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let starting_values: Vec<_> = input
        .trim()
        .split(",")
        .filter_map(|v| v.parse().ok())
        .collect();

    println!("{:?} {}", starting_values, play_n_round(&starting_values));
}

// run tests in --release
#[cfg(test)]
mod test {
    use super::play_n_round;
    #[test]
    fn test_0_3_6() {
        assert_eq!(play_n_round(&[0, 3, 6]), 175594);
    }
    #[test]
    fn test_1_3_2() {
        assert_eq!(play_n_round(&[1, 3, 2]), 2578);
    }
    #[test]
    fn test_2_1_3() {
        assert_eq!(play_n_round(&[2, 1, 3]), 3544142);
    }
    #[test]
    fn test_1_2_3() {
        assert_eq!(play_n_round(&[1, 2, 3]), 261214);
    }
    #[test]
    fn test_2_3_1() {
        assert_eq!(play_n_round(&[2, 3, 1]), 6895259);
    }
    #[test]
    fn test_3_2_1() {
        assert_eq!(play_n_round(&[3, 2, 1]), 18);
    }
    #[test]
    fn test_3_1_2() {
        assert_eq!(play_n_round(&[3, 1, 2]), 362);
    }
}
