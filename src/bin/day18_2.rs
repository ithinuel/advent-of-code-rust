use itertools::Itertools;
use std::io::BufRead;

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|s| compute(&mut s.into_bytes().into_iter().filter(|&b| b != b' ')))
            .sum::<usize>()
    );
}

fn compute(line: &mut impl Iterator<Item = u8>) -> usize {
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
        .ok()
        .expect("Invalid expression")
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
            231
        );
    }
    #[test]
    fn example2() {
        let input = b"2 * 3 + (4 * 5)";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            46
        );
    }
    #[test]
    fn example3() {
        let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            1445
        );
    }
    #[test]
    fn example4() {
        let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            669060
        );
    }
    #[test]
    fn example5() {
        let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(
            compute(&mut input.iter().copied().filter(|&b| b != b' ')),
            23340
        );
    }
}
