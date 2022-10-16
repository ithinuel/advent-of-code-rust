use yaah::*;

mod snailfish;
use itertools::Itertools;
use snailfish::SnailFishNumber;

#[aoc_generator(day18)]
fn gen(input: &str) -> Vec<SnailFishNumber> {
    input.lines().map(SnailFishNumber::parse).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[SnailFishNumber]) -> Option<usize> {
    input
        .iter()
        .cloned()
        .reduce(|a, b| a + b)
        .as_ref()
        .map(SnailFishNumber::magnitude)
}

#[aoc(day18, part2)]
fn part2(input: &[SnailFishNumber]) -> Option<usize> {
    input
        .iter()
        .cloned()
        .tuple_combinations()
        .flat_map(|(a, b)| [(a.clone() + b.clone()).magnitude(), (b + a).magnitude()].into_iter())
        .max()
}

#[cfg(test)]
mod test {
    use super::SnailFishNumber;

    const EXAMPLE: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn gen() {
        assert_eq!(
            vec![
                SnailFishNumber::new(1, 2),
                SnailFishNumber::new(8, SnailFishNumber::new(1, 2))
            ],
            super::gen(
                r"[1,2]
[8,[1,2]]"
            )
        );
    }

    #[test]
    fn part1() {
        assert_eq!(Some(4140), super::part1(&super::gen(EXAMPLE)));
    }
}
