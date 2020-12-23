fn play_n_round(starting_values: &[usize]) -> usize {
    let mut memory = starting_values.to_vec();
    (starting_values.len()..2020).fold(0, |_, _| {
        let previous = *memory.last().unwrap();
        let last = memory[..memory.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, v)| v == previous)
            .last();

        let new = last.map(|(idx, _)| memory.len() - (idx + 1)).unwrap_or(0);
        memory.push(new);
        new
    })
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

#[cfg(test)]
mod test {
    use super::play_n_round;
    #[test]
    fn test_0_3_6() {
        assert_eq!(play_n_round(&[0, 3, 6]), 436);
    }
    #[test]
    fn test_1_3_2() {
        assert_eq!(play_n_round(&[1, 3, 2]), 1);
    }
    #[test]
    fn test_2_1_3() {
        assert_eq!(play_n_round(&[2, 1, 3]), 10);
    }
    #[test]
    fn test_1_2_3() {
        assert_eq!(play_n_round(&[1, 2, 3]), 27);
    }
    #[test]
    fn test_2_3_1() {
        assert_eq!(play_n_round(&[2, 3, 1]), 78);
    }
    #[test]
    fn test_3_2_1() {
        assert_eq!(play_n_round(&[3, 2, 1]), 438);
    }
    #[test]
    fn test_3_1_2() {
        assert_eq!(play_n_round(&[3, 1, 2]), 1836);
    }
}
