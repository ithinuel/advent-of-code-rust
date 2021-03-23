use aoc_runner_derive::*;
use itertools::Itertools;

fn compute_part1(line: &mut impl Iterator<Item = u8>) -> isize {
    // ( or operand
    let mut acc = match line.next() {
        Some(b'(') => compute_part1(line),
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
            Some(b'(') => compute_part1(line),
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

#[aoc(day18, part1)]
fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|s| compute_part1(&mut s.bytes().filter(|&b| b != b' ')))
        .sum()
}

fn compute_part2(line: &mut impl Iterator<Item = u8>) -> usize {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    fn prec(op: u8) -> isize {
        match op {
            b'+' => 1,
            b'*' => 0,
            b'(' => -1,
            _ => unreachable!(),
        }
    }
    fn colapse(postfix: &mut Vec<usize>, op: u8) {
        let operand1 = postfix.pop().expect("Invalid expression");
        let operand2 = postfix.pop().expect("Invalid expression");
        let result = match op {
            b'+' => operand1 + operand2,
            b'*' => operand1 * operand2,
            _ => unreachable!(),
        };
        postfix.push(result);
    }

    line.for_each(|c| {
        match c {
            b'(' => stack.push(b'('),
            b')' => {
                while let Some(b) = stack.pop() {
                    if b == b'(' {
                        break;
                    }
                    colapse(&mut postfix, b);
                }
            }
            b @ b'0'..=b'9' => postfix.push((b - b'0') as usize),
            b @ b'*' | b @ b'+' => {
                // b is an operator
                while let Some(top) = stack.pop() {
                    if prec(top) < prec(b) {
                        stack.push(top);
                        break;
                    }
                    colapse(&mut postfix, top);
                }
                stack.push(b)
            }
            _ => unreachable!(),
        }
    });
    stack
        .drain(..)
        .rev()
        .for_each(|op| colapse(&mut postfix, op));
    postfix
        .into_iter()
        .exactly_one()
        .expect("Invalid expression")
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| compute_part2(&mut s.bytes().filter(|&b| b != b' ')))
        .sum()
}

#[cfg(test)]
mod test_part1 {
    use super::compute_part1;
    #[test]
    fn one_operand() {
        let input = b"1";
        assert_eq!(compute_part1(&mut input.iter().copied()), 1);
    }
    #[test]
    fn addition() {
        let input = b"1+2";
        assert_eq!(compute_part1(&mut input.iter().copied()), 3);
    }

    #[test]
    fn example1() {
        let input = b"1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(
            compute_part1(&mut input.iter().copied().filter(|&b| b != b' ')),
            71
        );
    }
    #[test]
    fn example2() {
        let input = b"2 * 3 + (4 * 5)";
        assert_eq!(
            compute_part1(&mut input.iter().copied().filter(|&b| b != b' ')),
            26
        );
    }
    #[test]
    fn example3() {
        let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(
            compute_part1(&mut input.iter().copied().filter(|&b| b != b' ')),
            437
        );
    }
    #[test]
    fn example4() {
        let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(
            compute_part1(&mut input.iter().copied().filter(|&b| b != b' ')),
            12240
        );
    }
    #[test]
    fn example5() {
        let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(
            compute_part1(&mut input.iter().copied().filter(|&b| b != b' ')),
            13632
        );
    }
}

#[cfg(test)]
mod test_part2 {
    use super::compute_part2;
    #[test]
    fn one_operand() {
        let input = b"1";
        assert_eq!(compute_part2(&mut input.iter().copied()), 1);
    }
    #[test]
    fn addition() {
        let input = b"1+2";
        assert_eq!(compute_part2(&mut input.iter().copied()), 3);
    }

    #[test]
    fn example1() {
        let input = b"1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(
            compute_part2(&mut input.iter().copied().filter(|&b| b != b' ')),
            231
        );
    }
    #[test]
    fn example2() {
        let input = b"2 * 3 + (4 * 5)";
        assert_eq!(
            compute_part2(&mut input.iter().copied().filter(|&b| b != b' ')),
            46
        );
    }
    #[test]
    fn example3() {
        let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(
            compute_part2(&mut input.iter().copied().filter(|&b| b != b' ')),
            1445
        );
    }
    #[test]
    fn example4() {
        let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(
            compute_part2(&mut input.iter().copied().filter(|&b| b != b' ')),
            669060
        );
    }
    #[test]
    fn example5() {
        let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(
            compute_part2(&mut input.iter().copied().filter(|&b| b != b' ')),
            23340
        );
    }
}
