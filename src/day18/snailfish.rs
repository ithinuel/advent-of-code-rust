#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropagationDir {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Explosion {
    Detonate(u8, u8),
    Shockwave(u8, PropagationDir),
    Blown,
}

#[derive(Debug, PartialEq, Eq)]
struct Split;

#[derive(PartialEq, Eq, Clone)]
pub enum SFNElem {
    Leaf(u8),
    Branch(Box<SnailFishNumber>),
}
impl std::fmt::Debug for SFNElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(number) => write!(f, "{}", number),
            Self::Branch(child) => write!(f, "{:?}", child),
        }
    }
}
impl SFNElem {
    fn explode(&mut self, depth: usize) -> Option<Explosion> {
        match self {
            SFNElem::Leaf(_) => None,
            SFNElem::Branch(child) => match child.explode(depth + 1) {
                None => match (&child.a, &child.b) {
                    (&SFNElem::Leaf(a), &SFNElem::Leaf(b)) if depth >= 4 => {
                        *self = 0u8.into();
                        Some(Explosion::Detonate(a, b))
                    }
                    (_, _) if depth >= 4 => unreachable!(),
                    _ => None,
                },
                other => other,
            },
        }
    }
    fn split(&mut self) -> Option<Split> {
        match self {
            SFNElem::Branch(child) => child.split(),
            &mut SFNElem::Leaf(v) if v > 9 => {
                *self = SnailFishNumber::new(v / 2, (v + 1) / 2).into();
                Some(Split)
            }
            _ => None,
        }
    }

    fn propagate(&mut self, v: u8, direction: PropagationDir) {
        match self {
            SFNElem::Leaf(value) => *value += v,
            SFNElem::Branch(child) => match direction {
                PropagationDir::Left => child.a.propagate(v, direction),
                PropagationDir::Right => child.b.propagate(v, direction),
            },
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            &SFNElem::Leaf(v) => usize::from(v),
            SFNElem::Branch(b) => b.magnitude(),
        }
    }
    #[allow(dead_code)]
    fn branch(&self) -> Option<&SnailFishNumber> {
        match self {
            SFNElem::Branch(b) => Some(b),
            _ => None,
        }
    }
    #[allow(dead_code)]
    fn branch_mut(&mut self) -> Option<&mut SnailFishNumber> {
        match self {
            SFNElem::Branch(b) => Some(b),
            _ => None,
        }
    }
    #[allow(dead_code)]
    fn leaf(&self) -> Option<&u8> {
        match self {
            SFNElem::Leaf(v) => Some(v),
            _ => None,
        }
    }
    #[allow(dead_code)]
    fn leaf_mut(&mut self) -> Option<&mut u8> {
        match self {
            SFNElem::Leaf(v) => Some(v),
            _ => None,
        }
    }
}
impl From<u8> for SFNElem {
    fn from(val: u8) -> Self {
        Self::Leaf(val)
    }
}
impl From<SnailFishNumber> for SFNElem {
    fn from(fish: SnailFishNumber) -> Self {
        Self::Branch(Box::new(fish))
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SnailFishNumber {
    a: SFNElem,
    b: SFNElem,
}
impl std::fmt::Debug for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.a, self.b)
    }
}

impl SnailFishNumber {
    pub fn new<T, U>(a: T, b: U) -> Self
    where
        T: Into<SFNElem>,
        U: Into<SFNElem>,
    {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let (v, len) = Self::parse_internal(bytes, 0);
        assert!(len.is_empty(), "Invalid format");
        v
    }
    fn parse_internal(input: &[u8], depth: usize) -> (Self, &[u8]) {
        macro_rules! parse_sfnelem {
            ($tag:expr, $input:expr) => {{
                match $input {
                    [$tag, b, rest @ ..] if b.is_ascii_digit() => ((b - b'0').into(), rest),
                    [$tag, rest @ ..] => {
                        let (v, rest) = Self::parse_internal(rest, depth + 1);
                        (v.into(), rest)
                    }
                    _ => unreachable!("Invalid format"),
                }
            }};
        }

        let (a, input) = parse_sfnelem!(b'[', input);
        let (b, input) = parse_sfnelem!(b',', input);
        let rest = match input {
            [b']', rest @ ..] => rest,
            _ => unreachable!("Invalid format"),
        };
        (Self { a, b }, rest)
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(1).is_none() && self.split().is_none() {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<Explosion> {
        self.a
            .explode(depth)
            .map(|res| match res {
                Explosion::Detonate(a, b) => {
                    self.b.propagate(b, PropagationDir::Left);
                    Explosion::Shockwave(a, PropagationDir::Left)
                }
                Explosion::Shockwave(v, PropagationDir::Right) => {
                    self.b.propagate(v, PropagationDir::Left);
                    Explosion::Blown
                }
                other => other,
            })
            .or_else(|| {
                self.b.explode(depth).map(|res| match res {
                    Explosion::Detonate(a, b) => {
                        self.a.propagate(a, PropagationDir::Right);
                        Explosion::Shockwave(b, PropagationDir::Right)
                    }
                    Explosion::Shockwave(v, PropagationDir::Left) => {
                        self.a.propagate(v, PropagationDir::Right);
                        Explosion::Blown
                    }
                    other => other,
                })
            })
    }
    fn split(&mut self) -> Option<Split> {
        match self.a.split() {
            None => self.b.split(),
            s => s,
        }
    }
    pub fn magnitude(&self) -> usize {
        3 * self.a.magnitude() + 2 * self.b.magnitude()
    }
}
impl std::ops::Add for SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new = SnailFishNumber {
            a: self.into(),
            b: rhs.into(),
        };
        new.reduce();
        new
    }
}

#[cfg(test)]
mod test {
    use super::SnailFishNumber;

    #[test]
    fn reductions() {
        [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ]
        .into_iter()
        .for_each(|(sample, expect)| {
            let mut sample = SnailFishNumber::parse(sample);
            let expect = SnailFishNumber::parse(expect);
            sample.reduce();
            assert_eq!(expect, sample);
        })
    }

    #[test]
    fn sum_ops() {
        let a = SnailFishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailFishNumber::parse("[1,1]");
        let expect = SnailFishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(expect, a + b);
    }

    #[test]
    fn sum_list() {
        [
            (
                r"[1,1]
[2,2]
[3,3]
[4,4]",
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                r"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                r"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ]
        .iter()
        .for_each(|(list, expect)| {
            let expect = SnailFishNumber::parse(expect);
            let result = list
                .lines()
                .map(SnailFishNumber::parse)
                .reduce(|a, b| a + b);
            assert_eq!(Some(expect), result);
        });
    }

    #[test]
    fn magnitude() {
        [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ]
        .into_iter()
        .for_each(|(input, expect)| assert_eq!(expect, SnailFishNumber::parse(input).magnitude()))
    }

    #[test]
    #[should_panic = "Invalid format"]
    fn bad_input_missing_first() {
        SnailFishNumber::parse("[,3]");
    }
    #[test]
    #[should_panic = "Invalid format"]
    fn bad_input_missing_last() {
        SnailFishNumber::parse("[3,]");
    }
    #[test]
    #[should_panic = "Invalid format"]
    fn bad_input_missing_open_bracket() {
        SnailFishNumber::parse("[3,4]]");
    }
    #[test]
    #[should_panic = "Invalid format"]
    fn bad_input_missing_close_bracket() {
        SnailFishNumber::parse("[[4,3,2]");
    }
}
