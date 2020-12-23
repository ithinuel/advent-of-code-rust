use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Borders {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

impl From<(u32, u32, u32, u32)> for Borders {
    fn from(v: (u32, u32, u32, u32)) -> Self {
        Borders {
            top: v.0,
            right: v.1,
            bottom: v.2,
            left: v.3,
        }
    }
}
fn flip(v: u32) -> u32 {
    (0..10).fold(0, |acc, i| (acc << 1) | ((v >> i) & 1))
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let map: Vec<(_, Borders, _)> = input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id: u32 = lines
                .next()
                .and_then(|line| line[5..9].parse().ok())
                .expect("Invalid tile number");
            let image: Vec<u32> = lines
                .map(|line| {
                    line.chars()
                        .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
                })
                .collect();

            let top = image[0];
            let left = (0..10).fold(0, |acc, i| acc | ((image[i] & 0b10_0000_0000) >> (9 - i)));
            let bottom = image[9];
            let right = (0..10).fold(0, |acc, i| acc | ((image[i] & 1) << i));

            // precompute transformations
            (id, (top, right, bottom, left).into(), image)
        })
        .collect();

    // Possible transformation are composition of Rotation by 0, 90, 180 or 270° and possibly a
    // Vertical Flip (two flip cancel each other and H+V flip are equivalent to a 180° rotation),
    #[derive(Debug, Clone, Copy)]
    enum Rotation {
        None,
        CCW90,
        CCW180,
        CW90,
    }
    #[derive(Debug, Clone, Copy)]
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
    //
    let possible_match: Vec<_> = map
        .iter()
        .map(|(id, borders, _)| {
            (
                *id,
                map.iter()
                    .filter(|(id2, _, _)| id != id2)
                    .filter_map(|(id2, borders2, _)| {
                        if borders.top == borders2.bottom {
                            Some((id2, Position::Top, Rotation::None, Flip::None))
                        } else if borders.top == flip(borders2.left) {
                            Some((id2, Position::Top, Rotation::CCW90, Flip::None))
                        } else if borders.top == flip(borders2.top) {
                            Some((id2, Position::Top, Rotation::CCW180, Flip::None))
                        } else if borders.top == borders2.right {
                            Some((id2, Position::Top, Rotation::CW90, Flip::None))
                        } else if borders.top == flip(borders2.bottom) {
                            Some((id2, Position::Top, Rotation::None, Flip::Vertical))
                        } else if borders.top == borders2.left {
                            Some((id2, Position::Top, Rotation::CCW90, Flip::Vertical))
                        } else if borders.top == borders2.top {
                            Some((id2, Position::Top, Rotation::CCW180, Flip::Vertical))
                        } else if borders.top == flip(borders2.right) {
                            Some((id2, Position::Top, Rotation::CW90, Flip::Vertical))
                        } else if borders.right == borders2.left {
                            Some((id2, Position::Right, Rotation::None, Flip::None))
                        } else if borders.right == borders2.top {
                            Some((id2, Position::Right, Rotation::CCW90, Flip::None))
                        } else if borders.right == flip(borders2.right) {
                            Some((id2, Position::Right, Rotation::CCW180, Flip::None))
                        } else if borders.right == flip(borders2.bottom) {
                            Some((id2, Position::Right, Rotation::CW90, Flip::None))
                        } else if borders.right == flip(borders2.left) {
                            Some((id2, Position::Right, Rotation::None, Flip::Vertical))
                        } else if borders.right == flip(borders2.top) {
                            Some((id2, Position::Right, Rotation::CCW90, Flip::Vertical))
                        } else if borders.right == borders2.right {
                            Some((id2, Position::Right, Rotation::CCW180, Flip::Vertical))
                        } else if borders.right == borders2.bottom {
                            Some((id2, Position::Right, Rotation::CW90, Flip::Vertical))
                        } else if borders.bottom == borders2.top {
                            Some((id2, Position::Bottom, Rotation::None, Flip::None))
                        } else if borders.bottom == flip(borders2.right) {
                            Some((id2, Position::Bottom, Rotation::CCW90, Flip::None))
                        } else if borders.bottom == flip(borders2.bottom) {
                            Some((id2, Position::Bottom, Rotation::CCW180, Flip::None))
                        } else if borders.bottom == borders2.left {
                            Some((id2, Position::Bottom, Rotation::CW90, Flip::None))
                        } else if borders.bottom == flip(borders2.top) {
                            Some((id2, Position::Bottom, Rotation::None, Flip::Vertical))
                        } else if borders.bottom == borders2.right {
                            Some((id2, Position::Bottom, Rotation::CCW90, Flip::Vertical))
                        } else if borders.bottom == borders2.bottom {
                            Some((id2, Position::Bottom, Rotation::CCW180, Flip::Vertical))
                        } else if borders.bottom == flip(borders2.left) {
                            Some((id2, Position::Bottom, Rotation::CW90, Flip::Vertical))
                        } else if borders.left == borders2.right {
                            Some((id2, Position::Left, Rotation::None, Flip::None))
                        } else if borders.left == flip(borders2.top) {
                            Some((id2, Position::Left, Rotation::CCW90, Flip::None))
                        } else if borders.left == flip(borders2.left) {
                            Some((id2, Position::Left, Rotation::CCW180, Flip::None))
                        } else if borders.left == borders2.bottom {
                            Some((id2, Position::Left, Rotation::CW90, Flip::None))
                        } else if borders.left == flip(borders2.right) {
                            Some((id2, Position::Left, Rotation::None, Flip::Vertical))
                        } else if borders.left == borders2.top {
                            Some((id2, Position::Left, Rotation::CCW90, Flip::Vertical))
                        } else if borders.left == borders2.left {
                            Some((id2, Position::Left, Rotation::CCW180, Flip::Vertical))
                        } else if borders.left == flip(borders2.bottom) {
                            Some((id2, Position::Left, Rotation::CW90, Flip::Vertical))
                        } else {
                            None
                        }
                    })
                    .count(),
            )
        })
        .collect();
    println!(
        "{:?}",
        possible_match
            .iter()
            .filter(|(_, n)| n == &2)
            .map(|(id, _)| *id as u64)
            .product::<u64>()
    );
}
