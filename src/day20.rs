use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;
use itertools::Itertools;

mod tile_mapper;
use tile_mapper::*;

#[aoc_generator(day20)]
fn gen(input: &str) -> Option<HashMap<u32, Tile>> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id: u32 = lines
                .next()
                .and_then(|line| line[5..9].parse().ok())
                .expect("Invalid tile number");

            let tile = Tile::try_from(lines)?;

            Result::<_, &'static str>::Ok((id, tile))
        })
        .try_collect()
        .ok()
}

#[derive(Debug)]
enum RebuildError {
    EmptyTileMap,
}
fn rebuild_map(
    tiles: &HashMap<u32, Tile>,
) -> Result<HashMap<(i32, i32), (u32, Rotation, Flip)>, RebuildError> {
    let signatures: Vec<_> = tiles
        .iter()
        .map(|(id, tile)| (*id, tile.borders()))
        .flat_map(|(id, borders)| {
            [
                (Rotation::None, 0),
                (Rotation::CCW90, 1),
                (Rotation::CCW180, 2),
                (Rotation::CW90, 3),
            ]
            .into_iter()
            .cartesian_product([Flip::None, Flip::Vertical].into_iter())
            .map(move |((rot, rcnt), flip)| {
                let mut borders = borders.clone();
                (0..rcnt).for_each(|_| borders.rotate_ccw90());
                if flip == Flip::Vertical {
                    borders.vertical_flip();
                }
                (id, rot, flip, borders)
            })
        })
        .collect();

    // assert there no signature that matches twice
    let correlated: HashSet<_> = signatures
        .iter()
        .flat_map(|sig1| signatures.iter().map(move |sig2| (sig1, sig2)))
        .filter(|(sig1, sig2)| sig1.0 != sig2.0) // ignore self matches :P
        .filter_map(|(sig1, sig2)| {
            sig1.3
                .into_iter()
                .cartesian_product(sig2.3.into_iter())
                .filter(|((pos1, _), (pos2, _))| {
                    matches!(
                        (*pos1, *pos2),
                        (Position::Top, Position::Bottom)
                            | (Position::Bottom, Position::Top)
                            | (Position::Left, Position::Right)
                            | (Position::Right, Position::Left)
                    )
                }) // only consider facing sides (top-bottom, left-right)
                .filter_map(|((pos1, val1), (rot2, pos2))| {
                    (val1 == pos2).then(|| {
                        (
                            ((sig1.0, sig1.1, sig1.2), pos1),
                            ((sig2.0, sig2.1, sig2.2), rot2),
                        )
                    })
                })
                .at_most_one()
                .transpose()
        })
        .try_collect()
        .expect("match unicity assumption is broken");

    let first = tiles
        .keys()
        .min()
        .cloned()
        .ok_or(RebuildError::EmptyTileMap)?;
    // let's pick the no rotation-no flip state for the first one and use it as 0,0
    let mut mapped =
        std::iter::once(((first, Rotation::None, Flip::None), (0, 0))).collect::<HashMap<_, _>>();

    let mut unmapped_tiles: HashSet<_> =
        tiles.keys().cloned().filter(|&key| key != first).collect();

    while !unmapped_tiles.is_empty() {
        let matched_with_map: HashSet<_> = correlated
            .iter()
            .filter(|(_, ((b, _, _), _))| unmapped_tiles.contains(b))
            .filter(|((a, _), _)| mapped.contains_key(a))
            .collect();

        //println!("{:?}", matched_with_map);
        matched_with_map
            .into_iter()
            .for_each(|((from, pos), (to, _))| {
                unmapped_tiles.remove(&to.0);
                let mapped_coords = mapped[from];
                let to_coords = match pos {
                    Position::Top => (mapped_coords.0, mapped_coords.1 - 1),
                    Position::Right => (mapped_coords.0 + 1, mapped_coords.1),
                    Position::Bottom => (mapped_coords.0, mapped_coords.1 + 1),
                    Position::Left => (mapped_coords.0 - 1, mapped_coords.1),
                };
                mapped.insert(*to, to_coords);
            });
    }

    Ok(mapped.into_iter().map(|(k, v)| (v, k)).collect())
}

fn get_corners(map: &HashMap<(i32, i32), (u32, Rotation, Flip)>) -> ((i32, i32), (i32, i32)) {
    map.keys().fold(
        (
            (i32::max_value(), i32::max_value()),
            (i32::min_value(), i32::min_value()),
        ),
        |(min, max), v| {
            (
                (i32::min(min.0, v.0), i32::min(min.1, v.1)),
                (i32::max(max.0, v.0), i32::max(max.1, v.1)),
            )
        },
    )
}

#[aoc(day20, part1)]
fn part1(tiles: &HashMap<u32, Tile>) -> Option<u64> {
    let map = rebuild_map(tiles).ok()?;
    let ((xmin, ymin), (xmax, ymax)) = get_corners(&map);

    Some(
        [xmin, xmax]
            .into_iter()
            .cartesian_product([ymin, ymax].into_iter())
            .map(|coord| map[&coord].0 as u64)
            .product(),
    )
}

#[aoc(day20, part2)]
fn part2(tiles: &HashMap<u32, Tile>) -> Option<usize> {
    let map = rebuild_map(tiles).ok()?;
    let image = Image::new(tiles, &map);

    [Flip::None, Flip::Vertical]
        .into_iter()
        .cartesian_product([
            Rotation::None,
            Rotation::CCW90,
            Rotation::CCW180,
            Rotation::CW90,
        ])
        .find_map(|(flip, rot)| {
            let mut image = image.clone();
            image.rotate(rot);
            image.flip(flip);

            let cnt = image.find_monster().count();
            (cnt != 0).then(|| {
                image
                    .0
                    .values()
                    .flat_map(|(_, tile)| tile.image().iter().skip(1).take(8))
                    .map(|line| (line & 0x1FE).count_ones())
                    .sum::<u32>() as usize
                    - cnt * 15
            })
        })
}

#[cfg(test)]
mod test {
    pub const REF: &str = r".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###";

    use crate::day20::tile_mapper::Image;

    const TEST: &str = include_str!("../test20.txt");

    #[test]
    fn part1() {
        assert_eq!(
            Some(1951 * 3079 * 2971 * 1171),
            super::gen(TEST).as_ref().and_then(super::part1)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(Some(273), super::gen(TEST).as_ref().and_then(super::part2));
    }

    #[test]
    fn rebuild_image() {
        let tiles = super::gen(TEST).unwrap();
        let map = super::rebuild_map(&tiles).unwrap();
        let mut image = Image::new(&tiles, &map);

        image.rotate(super::Rotation::CCW180);
        image.flip(super::Flip::Vertical);

        assert_eq!(REF, &format!("{:?}", image));
    }
}
