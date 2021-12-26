use std::collections::HashMap;

use arrayvec::ArrayVec;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Location {
    // can take a value from 0..=3
    // (room_id, room_slot)
    Room(u8, u8),

    // can take a value from 0..=4
    Corridor(u8),
}
impl Location {
    pub fn to_cell(self) -> usize {
        usize::from(match self {
            Self::Room(id, _) => id,
            Self::Corridor(id) => id,
        })
    }
    pub fn to_x(self) -> usize {
        usize::from(match self {
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
        })
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Target {
    Room(usize),
    Corridor(usize),
}
impl Target {
    pub fn from_coords(x: usize) -> Self {
        if x == 1 {
            Self::Corridor(0)
        } else if x == 11 {
            Self::Corridor(6)
        } else {
            let id = x / 2;
            if (x % 2) == 1 {
                Self::Room(id - 1)
            } else {
                Self::Corridor(id)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move {
    pub who: u8,
    pub from: Location,
    pub to: Location,
}
impl Move {
    pub fn cost(&self) -> usize {
        use Location::*;
        let scale = 10usize.pow(u32::from(self.who));
        (match (self.from, self.to) {
            (Room(_, slot), Corridor(_)) | (Corridor(_), Room(_, slot)) => {
                (slot + 1) + ((self.from.to_x() as i8) - (self.to.to_x() as i8)).abs() as u8
            }
            (Room(_, slot1), Room(_, slot2)) => {
                (slot1 + slot2 + 2)
                    + ((self.from.to_x() as i8) - (self.to.to_x() as i8)).abs() as u8
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
            .filter(|&(rid, r)| r.iter().any(|&a| rid != usize::from(a)))
            .filter_map(|(rid, r)| {
                r.last()
                    .map(|&who| (who, Location::Room(rid as u8, (DEPTH - r.len()) as u8)))
            })
            .chain(
                self.corridor
                    .iter()
                    .enumerate()
                    .filter_map(|(c, maybe_who)| {
                        maybe_who.map(|who| (who, Location::Corridor(c as u8)))
                    }),
            )
    }
    fn possible_moves(&self, who: u8, from: Location) -> impl Iterator<Item = Move> + '_ {
        let current_x = from.to_x();

        fn take_while_map<'a, const DEPTH: usize>(
            burrow: &'a Burrow<DEPTH>,
            who: u8,
            from: Location,
            it: impl Iterator<Item = usize> + 'a,
        ) -> impl Iterator<Item = Location> + 'a {
            it.map(Target::from_coords)
                // can't pass an occupied slot
                .take_while(|loc| match loc {
                    Target::Room(_) => true,
                    &Target::Corridor(id) => burrow.corridor[id].is_none(),
                })
                // exclude coridor to coridor movements
                .filter(move |&to| {
                    !matches!((from, to), (Location::Corridor(_), Target::Corridor(_)))
                })
                // only move to target room if not occupied by another type
                .filter_map(move |to| match to {
                    Target::Room(id) if id != usize::from(who) => None,
                    Target::Room(id) => {
                        let room = &burrow.rooms[id];
                        if room.iter().any(|&b| usize::from(b) != id) {
                            None
                        } else {
                            Some(Location::Room(id as u8, (DEPTH - 1 - room.len()) as u8))
                        }
                    }
                    Target::Corridor(id) => Some(Location::Corridor(id as u8)),
                })
        }

        take_while_map(self, who, from, (1..=(current_x - 1)).rev())
            .chain(take_while_map(self, who, from, (current_x + 1)..=11))
            .map(move |to| Move { who, from, to })
    }

    pub fn apply(&mut self, mv: &Move) {
        let (from_cell, to_cell) = (mv.from.to_cell(), mv.to.to_cell());
        match mv.from {
            Location::Corridor(_) => self.corridor[from_cell] = None,
            Location::Room(_, _) => {
                self.rooms[from_cell].pop();
            }
        }
        match mv.to {
            Location::Corridor(_) => self.corridor[to_cell] = Some(mv.who),
            Location::Room(_, _) => self.rooms[to_cell].push(mv.who),
        }
    }

    fn solve(
        &self,
        target: &Self,
        recurse: &dyn Fn(Self) -> HashMap<usize, Vec<Move>>,
    ) -> HashMap<usize, Vec<Move>> {
        use either::Either::*;
        self.requires_relocation()
            .flat_map(|(who, from)| self.possible_moves(who, from))
            .flat_map(|action| {
                let mut burrow = self.clone();
                burrow.apply(&action);
                let cost = action.cost();

                (if burrow == *target {
                    let mut v = Vec::with_capacity(25);
                    v.push(action);
                    Left(Some((cost, v)))
                } else {
                    Right(recurse(burrow).into_iter().map(move |(c, mut moves)| {
                        moves.push(action);
                        (c + cost, moves)
                    }))
                })
                .into_iter()
            })
            .collect()
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
pub fn solve2(burrow: Burrow<2>) -> HashMap<usize, Vec<Move>> {
    burrow.solve(&*TARGET2, &solve2)
}
#[cached::proc_macro::cached]
pub fn solve4(burrow: Burrow<4>) -> HashMap<usize, Vec<Move>> {
    burrow.solve(&*TARGET4, &solve4)
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
            who: 0,
            from: Room(1, 1),
            to: Room(3, 1),
        };
        assert_eq!(8, dut1.cost());
    }

    #[test]
    fn cost_corridor_to_room() {
        let dut1 = Move {
            who: 0,
            from: Corridor(2),
            to: Room(3, 1),
        };
        assert_eq!(7, dut1.cost());
    }
}
