use yaah::*;
use arrayvec::ArrayVec;
use itertools::Itertools;

type Map = ArrayVec<ArrayVec<u8, 10>, 10>;

#[aoc_generator(day11)]
fn gen(input: &str) -> Map {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn illuminate(map: &mut Map, (l, c): (usize, usize), do_light: bool) {
    let me = match map.get_mut(l).and_then(|l| l.get_mut(c)) {
        Some(0) => return,
        Some(b) => b,
        None => return,
    };

    if do_light {
        *me += 1;
    }
    if *me >= 10 {
        *me = 0;
        (-1..=1).cartesian_product(-1..=1).for_each(|(dl, dc)| {
            let l = (l as isize).wrapping_add(dl) as usize;
            let c = (c as isize).wrapping_add(dc) as usize;
            illuminate(map, (l, c), true);
        });
    }
}

fn step(map: &mut Map) {
    // add 1 to each
    for (l, c) in (0..10).cartesian_product(0..10) {
        map[l][c] += 1;
    }
    // any that reached 10 flashes
    for coords in (0..10).cartesian_product(0..10) {
        illuminate(map, coords, false);
    }
}

#[aoc(day11, part1)]
fn part1(input: &Map) -> usize {
    let mut input = input.clone();
    (0..100)
        .map(|_| {
            step(&mut input);
            input
                .iter()
                .flat_map(|l| l.iter())
                .filter(|&&b| b == 0)
                .count()
        })
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &Map) -> Option<usize> {
    let mut map = input.clone();
    (1..5000).find(|_| {
        step(&mut map);

        map.iter().flat_map(|l| l.iter()).all(|&b| b == 0)
    })
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const STEPS: &str = r"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637

8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848

0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000

2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211

4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322

5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433

6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644

7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755

9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976

0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000";

    #[test]
    fn step() {
        let mut input = gen(EXAMPLE);
        STEPS.split("\n\n").for_each(|step| {
            let bstep = gen(step);
            super::step(&mut input);
            assert_eq!(bstep, input);
        });
    }

    #[test]
    fn part1() {
        assert_eq!(1656, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(Some(195), super::part2(&gen(EXAMPLE)));
    }
}
