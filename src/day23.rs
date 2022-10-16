use yaah::*;

mod burrow;
use burrow::*;

#[aoc_generator(day23)]
fn gen(input: &str) -> Burrow<2> {
    Burrow::from(input)
}

#[aoc(day23, part1)]
fn part1(burrow: &Burrow<2>) -> Option<usize> {
    solve2(burrow.clone()).map(|(k, v)| {
        let mut burrow = burrow.clone();
        v.into_iter().rev().for_each(|mv| {
            burrow.apply(&mv);
            //println!("{:?}", mv);
            //println!("{:?}", burrow);
        });
        k
    })
}

#[aoc(day23, part2)]
fn part2(burrow: &Burrow<2>) -> Option<usize> {
    let burrow = Burrow::<4>::from(burrow);
    solve4(burrow.clone()).map(|(k, v)| {
        let mut burrow = burrow.clone();
        v.into_iter().rev().for_each(|mv| {
            burrow.apply(&mv);
            //println!("{:?}: {}", mv, mv.cost());
            //println!("{:?}", burrow);
        });
        k
    })
}

#[cfg(test)]
mod test {
    use super::Burrow;
    use arrayvec::ArrayVec;

    const EXAMPLE: &str = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn gen() {
        let expect = Burrow {
            rooms: [
                ArrayVec::from([0, 1]),
                ArrayVec::from([3, 2]),
                ArrayVec::from([2, 1]),
                ArrayVec::from([0, 3]),
            ],
            corridor: [None, None, None, None, None, None, None],
        };

        assert_eq!(expect, super::gen(EXAMPLE));
    }
    #[test]
    fn part1() {
        assert_eq!(Some(12521), super::part1(&super::gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(Some(44169), super::part2(&super::gen(EXAMPLE)));
    }
}
