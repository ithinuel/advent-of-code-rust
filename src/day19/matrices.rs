pub type Coord = (i32, i32, i32);

pub fn rotations() -> impl Iterator<Item = fn(Coord) -> Coord> {
    // x increases is ahead
    [
        // front facing, rotate around x
        |(x, y, z): Coord| (x, y, z),
        |(x, y, z): Coord| (x, z, -y),
        |(x, y, z): Coord| (x, -y, -z),
        |(x, y, z): Coord| (x, -z, y),
        // looking one side, rotate around y
        |(x, y, z): Coord| (y, -x, z),
        |(x, y, z): Coord| (-z, -x, y),
        |(x, y, z): Coord| (-y, -x, -z),
        |(x, y, z): Coord| (z, -x, -y),
        // looking back, rotate around x
        |(x, y, z): Coord| (-x, -y, z),
        |(x, y, z): Coord| (-x, z, y),
        |(x, y, z): Coord| (-x, y, -z),
        |(x, y, z): Coord| (-x, -z, -y),
        // looking the other side, rotate around y
        |(x, y, z): Coord| (-y, x, z),
        |(x, y, z): Coord| (-z, x, -y),
        |(x, y, z): Coord| (y, x, -z),
        |(x, y, z): Coord| (z, x, y),
        // looking up, rotate around z
        |(x, y, z): Coord| (-z, y, x),
        |(x, y, z): Coord| (y, z, x),
        |(x, y, z): Coord| (z, -y, x),
        |(x, y, z): Coord| (-y, -z, x),
        // looking down, rorate around z
        |(x, y, z): Coord| (z, y, -x),
        |(x, y, z): Coord| (y, -z, -x),
        |(x, y, z): Coord| (-y, z, -x),
        |(x, y, z): Coord| (-z, -y, -x),
    ]
    .into_iter()
}
