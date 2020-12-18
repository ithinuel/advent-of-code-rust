use std::io::BufRead;

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|s| compute(&mut s.into_bytes().into_iter().filter(|&b| b != b' ')))
            .sum::<isize>()
    );
}

fn compute(line: &mut impl Iterator<Item = u8>) -> isize {
    // ( or operand
    let mut acc = match line.next() {
        Some(b'(') => compute(line),
        Some(b) if (b'0'..=b'9').contains(&b) => (b - b'0') as isize,
        _ => unreachable!(),
    };
    loop {
        // ) or operator
        let operator = match line.next() {
            Some(b')') | None => return acc,
            Some(b) => b,
        };
        // ( or operand
        let operand = match line.next() {
            Some(b'(') => compute(line),
            Some(b) if (b'0'..=b'9').contains(&b) => (b - b'0') as isize,
            _ => unreachable!(),
        };
        match operator {
            b'+' => acc += operand,
            b'-' => acc -= operand,
            b'*' => acc *= operand,
            b'/' => acc /= operand,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::compute;
    #[test]
    fn one_operand() {
        let input = b"1";
        assert_eq!(compute(&mut input.iter().copied()), 1);
    }
    #[test]
    fn addition() {
        let input = b"1+2";
        assert_eq!(compute(&mut input.iter().copied()), 3);
    }

    #[test]
    fn example1() {
        let input = b"1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            71
        );
    }
    #[test]
    fn example2() {
        let input = b"2 * 3 + (4 * 5)";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            26
        );
    }
    #[test]
    fn example3() {
        let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            437
        );
    }
    #[test]
    fn example4() {
        let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            12240
        );
    }
    #[test]
    fn example5() {
        let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            13632
        );
    }
}
