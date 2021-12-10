use aoc_runner_derive::*;

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let mut stack = Vec::new();
            for b in l.bytes() {
                if let b'(' | b'<' | b'{' | b'[' = b {
                    stack.push(b);
                } else {
                    let is_ok = stack.pop().map(|b2| match (b2, b) {
                        (b'(', b')') | (b'{', b'}') | (b'[', b']') | (b'<', b'>') => true,
                        _ => false,
                    });
                    if !is_ok.unwrap_or(true) {
                        return Some(b);
                    }
                }
            }
            None
        })
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1() {
        assert_eq!(26397, super::part1(EXAMPLE));
    }
}
