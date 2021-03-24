use aoc_runner_derive::*;
use itertools::Itertools;

fn recursive(line: &mut impl Iterator<Item = u8>) -> isize {
    // ( or operand
    let mut acc = match line.next() {
        Some(b'(') => recursive(line),
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
            Some(b'(') => recursive(line),
            Some(b) if (b'0'..=b'9').contains(&b) => (b - b'0') as isize,
            _ => unreachable!(),
        };
        match operator {
            b'+' => acc += operand,
            //b'-' => acc -= operand,
            b'*' => acc *= operand,
            //b'/' => acc /= operand,
            _ => unreachable!(),
        }
    }
}

#[aoc(day18, part1)]
fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|s| recursive(&mut s.bytes().filter(|&b| b != b' ')))
        .sum()
}

fn iterative(line: &mut impl Iterator<Item = u8>) -> isize {
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
    fn colapse(postfix: &mut Vec<isize>, op: u8) {
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
            b @ b'0'..=b'9' => postfix.push((b - b'0') as isize),
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
            _ => {}
            //b if b.is_ascii_whitespace() => {}
            //b => unreachable!("unexpected: {}", b),
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
fn part2(input: &str) -> isize {
    input.lines().map(|s| iterative(&mut s.bytes())).sum()
}

#[cfg(test)]
mod test_recursive {
    use super::recursive;

    const EXAMPLES: [(isize, &[u8]); 6] = [
        (71, b"1 + 2 * 3 + 4 * 5 + 6"),
        (51, b"1 + (2 * 3) + (4 * (5 + 6))"),
        (26, b"2 * 3 + (4 * 5)"),
        (437, b"5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        (12240, b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        (13632, b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    ];

    #[test]
    fn one_operand() {
        let input = b"1";
        assert_eq!(
            recursive(&mut input.iter().copied().filter(|b| !b.is_ascii_whitespace())),
            1
        );
    }
    #[test]
    fn addition() {
        let input = b"1+2";
        assert_eq!(
            recursive(&mut input.iter().copied().filter(|b| !b.is_ascii_whitespace())),
            3
        );
    }

    #[test]
    fn examples() {
        for &(expected, input) in EXAMPLES.iter() {
            assert_eq!(
                expected,
                recursive(&mut input.iter().copied().filter(|b| !b.is_ascii_whitespace()))
            );
        }
    }
}

#[cfg(test)]
mod test_interative {
    use super::iterative;

    const EXAMPLES: [(isize, &[u8]); 6] = [
        (231, b"1 + 2 * 3 + 4 * 5 + 6"),
        (51, b"1 + (2 * 3) + (4 * (5 + 6))"),
        (46, b"2 * 3 + (4 * 5)"),
        (1445, b"5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        (669060, b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        (23340, b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    ];

    #[test]
    fn one_operand() {
        let input = b"1";
        assert_eq!(iterative(&mut input.iter().copied()), 1);
    }
    #[test]
    fn addition() {
        let input = b"1+2";
        assert_eq!(iterative(&mut input.iter().copied()), 3);
    }

    #[test]
    fn examples() {
        for &(expected, input) in EXAMPLES.iter() {
            assert_eq!(expected, iterative(&mut input.iter().copied()));
        }
    }
}
