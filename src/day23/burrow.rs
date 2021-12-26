use arrayvec::ArrayVec;
use either::Either;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Location {
    // can take a value from 0..=3
    // (room_id, room_slot)
    Room(usize, usize),

    // can take a value from 0..=4
    Corridor(usize),
}
impl Location {
    pub fn to_x(self) -> usize {
        match self {
            Self::Room(id, _) => 3 + id * 2,
            Self::Corridor(id) => {
                if id == 0 {
                    1
                } else if id == 6 {
                    11
                } else {
                    id * 2
                }
            }
        }
    }
    pub fn from_coords(x: usize, y: usize) -> Self {
        if x == 1 {
            Self::Corridor(0)
        } else if x == 11 {
            Self::Corridor(6)
        } else {
            let id = x / 2;
            if (x % 2) == 1 {
                Self::Room(id - 1, y)
            } else {
                Self::Corridor(id)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move {
    pub types: u8,
    pub from: Location,
    pub to: Location,
}
impl Move {
    pub fn cost(&self) -> usize {
        use Location::*;
        let scale = 10usize.pow(u32::from(self.types));
        (match (self.from, self.to) {
            (Room(_, slot), Corridor(_)) | (Corridor(_), Room(_, slot)) => {
                (slot + 1)
                    + ((self.from.to_x() as isize) - (self.to.to_x() as isize)).abs() as usize
            }
            (Room(_, slot1), Room(_, slot2)) => {
                (slot1 + slot2 + 2)
                    + ((self.from.to_x() as isize) - (self.to.to_x() as isize)).abs() as usize
            }
            _ => unreachable!("Impossible move"),
        }) as usize
            * scale
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Burrow<const DEPTH: usize> {
    pub rooms: [ArrayVec<u8, DEPTH>; 4],
    pub corridor: [Option<u8>; 7],
}
impl<const DEPTH: usize> Burrow<DEPTH> {
    fn requires_relocation(&self) -> impl Iterator<Item = (u8, Location)> + '_ {
        self.rooms
            .iter()
            .enumerate()
            // ignore rooms where all occupants are in the right place
            .filter(|&(rid, r)| r.iter().any(|&a| rid != usize::from(a)))
            // pick the top most occupant
            .filter_map(|(rid, r)| {
                r.last()
                    .map(|&types| (types, Location::Room(rid, DEPTH - r.len())))
            })
            // pick anyone lingering in the corridor
            .chain(
                self.corridor
                    .iter()
                    .enumerate()
                    .filter_map(|(c, maybe_types)| {
                        maybe_types.map(|types| (types, Location::Corridor(c)))
                    }),
            )
    }
    fn list_accessible_location_in_range<'a>(
        &'a self,
        from: Location,
        target_room: usize,
        it: impl Iterator<Item = usize> + 'a,
    ) -> impl Iterator<Item = Location> + 'a {
        // Y axis isn't used yet.
        it.map(|x| Location::from_coords(x, 0))
            // can't pass an occupied slot
            .take_while(|loc| match loc {
                Location::Room(_, _) => true,
                &Location::Corridor(id) => self.corridor[id].is_none(),
            })
            // exclude coridor to coridor movements
            .filter(move |&to| {
                !matches!((from, to), (Location::Corridor(_), Location::Corridor(_)))
            })
            .filter_map(move |to| match to {
                // ignore moves targetting other rooms
                Location::Room(id, _) if id != target_room => None,
                Location::Room(id, _) => {
                    let room = &self.rooms[id];
                    // only move to target room if not occupied by another type
                    (!room.iter().any(|&b| usize::from(b) != id)).then(|| {
                        // update Y axis
                        Location::Room(id, DEPTH - 1 - room.len())
                    })
                }
                Location::Corridor(_) => Some(to),
            })
    }
    fn possible_moves(&self, types: u8, from: Location) -> impl Iterator<Item = Move> + '_ {
        let current_x = from.to_x();
        let target_room = usize::from(types);

        // check to the left
        self.list_accessible_location_in_range(from, target_room, (1..=(current_x - 1)).rev())
            // check to the right
            .chain(self.list_accessible_location_in_range(from, target_room, (current_x + 1)..=11))
            // create a move of the destinations
            .map(move |to| Move { types, from, to })
    }

    pub fn apply(&mut self, mv: &Move) {
        match mv.from {
            Location::Corridor(id) => self.corridor[id] = None,
            Location::Room(id, _) => {
                self.rooms[id].pop();
            }
        }
        match mv.to {
            Location::Corridor(id) => self.corridor[id] = Some(mv.types),
            Location::Room(id, _) => self.rooms[id].push(mv.types),
        }
    }

    /// There may be multiple solution to reach the target but we only care about the shortest.
    /// For performance boost, use a memoized wrapper to this function as the "recurse"
    /// argument.
    fn shortest(
        &self,
        target: &Self,
        recurse: &dyn Fn(Self) -> Option<(usize, Vec<Move>)>,
    ) -> Option<(usize, Vec<Move>)> {
        self.requires_relocation()
            .flat_map(|(types, from)| self.possible_moves(types, from))
            .flat_map(|action| {
                let mut burrow = self.clone();
                burrow.apply(&action);
                let cost = action.cost();

                (if burrow == *target {
                    let mut v = Vec::with_capacity(25);
                    v.push(action);
                    Either::Left(Some((cost, v)))
                } else {
                    Either::Right(recurse(burrow).into_iter().map(move |(c, mut moves)| {
                        moves.push(action);
                        (c + cost, moves)
                    }))
                })
                .into_iter()
            })
            .min_by_key(|&(cost, _)| cost)
    }
}

lazy_static::lazy_static! {
    static ref TARGET2: Burrow<2> = {
        Burrow {
            rooms: [
                ArrayVec::from([0, 0]),
                ArrayVec::from([1, 1]),
                ArrayVec::from([2, 2]),
                ArrayVec::from([3, 3]),
            ],
            corridor: [None, None, None, None, None, None, None],
        }
    };
    pub static ref TARGET4: Burrow<4> = {
        Burrow {
            rooms: [
                ArrayVec::from([0, 0, 0, 0]),
                ArrayVec::from([1, 1, 1, 1]),
                ArrayVec::from([2, 2, 2, 2]),
                ArrayVec::from([3, 3, 3, 3]),
            ],
            corridor: [None, None, None, None, None, None, None],
        }
    };
}

#[cached::proc_macro::cached]
pub fn solve2(burrow: Burrow<2>) -> Option<(usize, Vec<Move>)> {
    burrow.shortest(&*TARGET2, &solve2)
}
#[cached::proc_macro::cached]
pub fn solve4(burrow: Burrow<4>) -> Option<(usize, Vec<Move>)> {
    burrow.shortest(&*TARGET4, &solve4)
}

impl<const DEPTH: usize> std::fmt::Debug for Burrow<DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.corridor[0].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[1].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[2].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[3].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[4].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[5].map(|b| (b + b'A') as char).unwrap_or('.'),
            self.corridor[6].map(|b| (b + b'A') as char).unwrap_or('.'),
        )?;
        (0..DEPTH).rev().try_for_each(|d| {
            let decorator = if d == (DEPTH - 1) { "##" } else { "  " };
            writeln!(
                f,
                "{}#{}#{}",
                decorator,
                (0..4)
                    .map(|r| {
                        let r = &self.rooms[r];
                        r.get(d).map(|b| (b'A' + b) as char).unwrap_or(' ')
                    })
                    .join("#"),
                decorator.trim()
            )
        })?;
        write!(f, "  #########")
    }
}
impl std::convert::From<&str> for Burrow<2> {
    fn from(input: &str) -> Self {
        const ROOM: ArrayVec<u8, 2> = ArrayVec::new_const();
        let mut rooms = [ROOM; 4];

        let lines = input.lines().skip(2).take(2).collect_vec();
        lines.into_iter().rev().for_each(|l| {
            l.trim()
                .split('#')
                .filter_map(|l| l.bytes().next())
                .enumerate()
                .for_each(|(room, b)| rooms[room].push(b - b'A'))
        });

        Self {
            rooms,
            corridor: [None, None, None, None, None, None, None],
        }
    }
}

impl std::convert::From<&Burrow<2>> for Burrow<4> {
    fn from(from: &Burrow<2>) -> Self {
        let rooms = [
            ArrayVec::from([from.rooms[0][0], 3, 3, from.rooms[0][1]]),
            ArrayVec::from([from.rooms[1][0], 1, 2, from.rooms[1][1]]),
            ArrayVec::from([from.rooms[2][0], 0, 1, from.rooms[2][1]]),
            ArrayVec::from([from.rooms[3][0], 2, 0, from.rooms[3][1]]),
        ];
        Burrow {
            rooms,
            corridor: [None, None, None, None, None, None, None],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Burrow;
    use super::Location::*;
    use super::Move;

    const EXAMPLE: &str = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn burrow_prints() {
        assert_eq!(EXAMPLE, &format!("{:?}", Burrow::from(EXAMPLE)));
    }

    #[test]
    fn cost_room_to_room() {
        let dut1 = Move {
            types: 0,
            from: Room(1, 1),
            to: Room(3, 1),
        };
        assert_eq!(8, dut1.cost());
    }

    #[test]
    fn cost_corridor_to_room() {
        let dut1 = Move {
            types: 0,
            from: Corridor(2),
            to: Room(3, 1),
        };
        assert_eq!(7, dut1.cost());
    }
}
