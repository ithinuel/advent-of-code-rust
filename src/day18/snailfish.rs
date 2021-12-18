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
    None,
}

#[derive(Debug, PartialEq, Eq)]
enum Split {
    Split,
    None,
}

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
    fn explode(&mut self, depth: usize) -> Explosion {
        match self {
            SFNElem::Leaf(_) => Explosion::None,
            SFNElem::Branch(child) => {
                match child.explode(depth + 1) {
                    Explosion::None => {}
                    other => return other,
                }
                //print!("{}: Branch({:?}) => ", depth, child);
                let r = if depth >= 4 {
                    match (&child.a, &child.b) {
                        (&SFNElem::Leaf(a), &SFNElem::Leaf(b)) => {
                            *self = 0u8.into();
                            Explosion::Detonate(a, b)
                        }
                        _ => unreachable!(),
                    }
                } else {
                    Explosion::None
                };
                //println!("{:?}", r);
                r
            }
        }
    }
    fn split(&mut self) -> Split {
        match self {
            SFNElem::Branch(child) => child.split(),
            &mut SFNElem::Leaf(v) if v > 9 => {
                *self = SnailFishNumber::new(v / 2, (v + 1) / 2).into();
                Split::Split
            }
            _ => Split::None,
        }
    }

    fn propagate(&mut self, v: u8, direction: PropagationDir) {
        //println!("propagate {} to the {:?} of {:?}", v, direction, self);
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
        assert_eq!(bytes.len(), len + 1);
        v
    }
    fn parse_internal(input: &[u8], depth: usize) -> (Self, usize) {
        ////println!("{}: parsing: {}", depth, String::from_utf8_lossy(input));
        assert_eq!(b'[', input[0]);
        let a_start = 1;
        let (a, a_end) = if input[a_start].is_ascii_digit() {
            let v = input[a_start] - b'0';
            (v.into(), a_start)
        } else {
            let (v, end) = Self::parse_internal(&input[1..], depth + 1);
            (v.into(), a_start + end)
        };
        ////println!("{}: parsed a: {:?}, end at {}", depth, a, a_end);
        assert_eq!(b',', input[a_end + 1]);
        let b_start = a_end + 2;
        let (b, b_end) = if input[b_start].is_ascii_digit() {
            let v = input[b_start] - b'0';
            (v.into(), b_start)
        } else {
            let (v, end) = Self::parse_internal(&input[b_start..], depth + 1);
            (v.into(), b_start + end)
        };
        ////println!("{}: parsed b: {:?}, end at {}", depth, b, b_end);
        assert_eq!(b']', input[b_end + 1]);

        (Self { a, b }, b_end + 1)
    }

    fn reduce(&mut self) {
        loop {
            match self.explode(1) {
                Explosion::None => {
                    if self.split() == Split::None {
                        break;
                    }
                }
                _other => {
                    //println!("explosion: {:?} -- {:?}", self, _other),
                }
            }
            //println!("=====")
        }
    }

    fn explode(&mut self, depth: usize) -> Explosion {
        //println!("{}: explode: {:?}", depth, self);
        match self.a.explode(depth) {
            Explosion::Detonate(a, b) => {
                self.b.propagate(b, PropagationDir::Left);
                return Explosion::Shockwave(a, PropagationDir::Left);
            }
            Explosion::Shockwave(v, PropagationDir::Right) => {
                self.b.propagate(v, PropagationDir::Left);
                return Explosion::Blown;
            }
            Explosion::None => {}
            other => return other,
        }
        match self.b.explode(depth) {
            Explosion::Detonate(a, b) => {
                self.a.propagate(a, PropagationDir::Right);
                return Explosion::Shockwave(b, PropagationDir::Right);
            }
            Explosion::Shockwave(v, PropagationDir::Left) => {
                self.a.propagate(v, PropagationDir::Right);
                return Explosion::Blown;
            }
            other => other,
        }
    }
    fn split(&mut self) -> Split {
        //println!("split: {:?}", self);
        match self.a.split() {
            Split::None => self.b.split(),
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
            let result = list.lines().map(SnailFishNumber::parse).reduce(|a, b| {
                //println!("====> {:?} + {:?}", a, b);
                a + b
            });
            assert_eq!(Some(expect), result);
            //println!("=====")
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
}
