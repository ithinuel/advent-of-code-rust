use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;
use std::iter::once;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    None,
    CCW90,
    CCW180,
    CW90,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Top,
    Right,
    Left,
    Bottom,
}
#[derive(Debug, Clone, Copy)]
enum Flip {
    None,
    Vertical,
}

#[derive(Debug)]
struct Tile {
    id: usize,
    image: Vec<u32>,
}
impl Tile {
    fn flip(&mut self) {
        let image = std::mem::replace(&mut self.image, Vec::new())
            .into_iter()
            .rev()
            .collect();
        self.image = image;
    }
    fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::CCW90 => {
                self.image = (0..10)
                    .map(|j| (0..10).fold(0, |acc, i| acc | ((self.image[i] >> j & 1) << (9 - i))))
                    .collect();
            }
            Rotation::CCW180 => {
                self.image = std::mem::replace(&mut self.image, Vec::new())
                    .into_iter()
                    .rev()
                    .map(|line| flip(line))
                    .collect();
            }
            Rotation::CW90 => {
                self.rotate(Rotation::CCW90);
                self.rotate(Rotation::CCW180);
            }
            Rotation::None => {}
        }
    }
    fn borders(&self) -> impl Iterator<Item = (Position, u32)> {
        let top = self.image[0];
        let left = (0..10).fold(0, |acc, i| {
            acc | ((self.image[i] & 0b10_0000_0000) >> (9 - i))
        });
        let bottom = self.image[9];
        let right = (0..10).fold(0, |acc, i| acc | ((self.image[i] & 1) << i));
        [
            (Position::Top, top),
            (Position::Left, left),
            (Position::Bottom, bottom),
            (Position::Right, right),
        ]
        .to_vec()
        .into_iter()
    }
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.image {
            for b in (0..10).rev() {
                let bit = ((line >> b) & 1) == 1;
                if bit {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn flip(v: u32) -> u32 {
    v.reverse_bits() >> 22
}

fn to_transformation(pos1: Position, pos2: Position, flipped: bool) -> (Rotation, Flip) {
    match (pos1, pos2, flipped) {
        (Position::Right, Position::Left, false)
        | (Position::Top, Position::Bottom, false)
        | (Position::Left, Position::Right, false)
        | (Position::Bottom, Position::Top, false) => (Rotation::None, Flip::None),
        (Position::Right, Position::Left, true)
        | (Position::Top, Position::Top, false)
        | (Position::Left, Position::Right, true)
        | (Position::Bottom, Position::Bottom, false) => (Rotation::None, Flip::Vertical),
        (Position::Right, Position::Top, false)
        | (Position::Top, Position::Left, true)
        | (Position::Left, Position::Bottom, false)
        | (Position::Bottom, Position::Right, true) => (Rotation::CCW90, Flip::None),
        (Position::Right, Position::Bottom, false)
        | (Position::Top, Position::Left, false)
        | (Position::Left, Position::Top, false)
        | (Position::Bottom, Position::Right, false) => (Rotation::CCW90, Flip::Vertical),
        (Position::Right, Position::Right, true)
        | (Position::Top, Position::Top, true)
        | (Position::Left, Position::Left, true)
        | (Position::Bottom, Position::Bottom, true) => (Rotation::CCW180, Flip::None),
        (Position::Right, Position::Right, false)
        | (Position::Top, Position::Bottom, true)
        | (Position::Left, Position::Left, false)
        | (Position::Bottom, Position::Top, true) => (Rotation::CCW180, Flip::Vertical),
        (Position::Right, Position::Bottom, true)
        | (Position::Top, Position::Right, false)
        | (Position::Left, Position::Top, true)
        | (Position::Bottom, Position::Left, false) => (Rotation::CW90, Flip::None),
        (Position::Right, Position::Top, true)
        | (Position::Top, Position::Right, true)
        | (Position::Left, Position::Bottom, true)
        | (Position::Bottom, Position::Left, true) => (Rotation::CW90, Flip::Vertical),
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let mut tiles: Vec<_> = input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines
                .next()
                .and_then(|line| line[5..9].parse().ok())
                .expect("Invalid tile number");
            let image: Vec<u32> = lines
                .map(|line| {
                    line.chars()
                        .enumerate()
                        .fold(0, |acc, (i, c)| acc | if c == '#' { 1 << i } else { 0 })
                })
                .collect();

            // extract borders

            Tile { id, image }
        })
        .collect();

    let map_width = if tiles.len() == 9 { 3 } else { 12 };

    let last = tiles.pop().expect("missing input");
    //println!("{:?}-{:?}", last.id, (Rotation::None, Flip::None));

    let mut map_borders: Vec<_> = last
        .borders()
        .map(|(side, sig)| (last.id, side, sig))
        .collect();
    let mut tile_loc_by_id: HashMap<_, _> = once((last.id, 0i16)).collect();
    let mut tile_by_loc: HashMap<_, _> = once((0i16, last)).collect();
    loop {
        // look for a match
        let result = map_borders
            .iter()
            .copied()
            .find_map(|(lhs_tile_id, lhs_side, lhs_sig)| {
                tiles.iter().enumerate().find_map(|(rhs_idx, rhs_tile)| {
                    rhs_tile.borders().find_map(|(rhs_side, rhs_sig)| {
                        let transformation = if lhs_sig == rhs_sig {
                            to_transformation(lhs_side, rhs_side, false)
                        } else if lhs_sig == flip(rhs_sig) {
                            to_transformation(lhs_side, rhs_side, true)
                        } else {
                            return None;
                        };
                        Some((lhs_tile_id, lhs_side, rhs_idx, transformation))
                    })
                })
            });

        // if no match then we're done.
        let (lhs_tile_id, lhs_side, rhs_idx, transformation) =
            if let Some((lhs_tile_id, lhs_side, rhs_idx, transformation)) = result {
                (lhs_tile_id, lhs_side, rhs_idx, transformation)
            } else {
                break;
            };

        // remove the tile from available pool and transform it as require.
        let mut tile = tiles.remove(rhs_idx);
        if let Flip::Vertical = transformation.1 {
            tile.flip();
        }
        tile.rotate(transformation.0);
        //println!("Tile {}: {:?}", tile.id, transformation);
        //print!("{}", tile);
        //println!();

        // borders after transformation
        let borders = tile.borders();
        let tile_id = tile.id;

        // add tile to the map.
        let match_loc = tile_loc_by_id[&lhs_tile_id];
        let location = match lhs_side {
            Position::Top => match_loc - (map_width as i16),
            Position::Left => match_loc - 1,
            Position::Right => match_loc + 1,
            Position::Bottom => match_loc + (map_width as i16),
        };

        tile_loc_by_id.insert(tile.id, location);
        tile_by_loc.insert(location, tile);

        // update the list of borders
        map_borders.extend(borders.map(|(side, sig)| (tile_id, side, sig)));
    }
    //println!("{:?}", map_borders);
    //println!("{:?}", tile_location_by_id);
    //println!("{:?}", tile_location_by_loc);

    assert_eq!(tile_by_loc.len(), map_width * map_width);

    let map: Vec<_> = tile_by_loc
        .keys()
        .copied()
        .minmax()
        .into_option()
        .map(|(min, max)| (min..=max))
        .expect("invalid map")
        .filter_map(|id| tile_by_loc.remove(&id))
        .collect();
    //println!("{:?}", map);

    // reconstitute the image
    let ref_map = &map;
    let map: Vec<Vec<bool>> = (0..map_width)
        .flat_map(|ty| {
            (0..8).map(move |dy| {
                (0..map_width)
                    .rev()
                    .flat_map(|tx| {
                        let loc = ty * map_width + tx;
                        let ref_tile = &ref_map[loc];

                        (0..8).map(move |dx| (ref_tile.image[1 + dy] >> (1 + dx)) & 1 == 1)
                    })
                    .collect::<Vec<bool>>()
            })
        })
        .collect();

    map.iter().for_each(|line| {
        line.iter()
            .for_each(|b| print!("{}", if *b { "#" } else { "." }));
        println!();
    })

    // Apply correlation to find the sea monsters.
}
