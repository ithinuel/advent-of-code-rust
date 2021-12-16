use aoc_runner_derive::*;
use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum Payload {
    Literals(u32),
    Operator {
        op: Operation,
        operands: Vec<Packet>,
    },
}

#[derive(Debug)]
struct Packet {
    version: u32,
    payload: Payload,
}
impl Packet {
    fn compute(&self) -> u32 {
        match &self.payload {
            Payload::Literals(v) => *v,
            Payload::Operator { op, operands } => {
                let mut operands = operands.iter().map(|op| op.compute());
                match op {
                    Operation::Sum => operands.sum(),
                    Operation::Product => operands.product(),
                    Operation::Minimum => operands.min().expect("No operands!"),
                    Operation::Maximum => operands.max().expect("No operands!"),
                    Operation::GreaterThan => {
                        let (a, b) = operands.next_tuple().expect("Missin operands");
                        assert_eq!(None, operands.next(), "Unexpected extra operand");
                        if a > b {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::LessThan => {
                        let (a, b) = operands.next_tuple().expect("Missin operands");
                        assert_eq!(operands.next(), None);
                        if a < b {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::EqualTo => {
                        let (a, b) = operands.next_tuple().expect("Missin operands");
                        assert_eq!(operands.next(), None);
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

fn read<T>(bits: usize, stream: &mut T) -> u32
where
    T: Iterator<Item = u32>,
    T: ?Sized,
{
    stream.take(bits).fold(0, |acc, b| acc << 1 | b)
}

fn parse_packet(it: &mut (dyn Iterator<Item = u32>)) -> Option<Packet> {
    let version = read(3, it);
    let type_id = read(3, it);
    let payload = match type_id {
        4 => {
            let mut literal = 0;
            while let Some(cont) = it.next() {
                let val = read(4, it);
                literal = literal * 16 + val;
                if cont == 0 {
                    break;
                }
            }
            Payload::Literals(literal)
        }
        op_code => {
            let op = match op_code {
                0 => Operation::Sum,
                1 => Operation::Product,
                2 => Operation::Minimum,
                3 => Operation::Maximum,
                5 => Operation::GreaterThan,
                6 => Operation::LessThan,
                7 => Operation::EqualTo,
                _ => unreachable!(),
            };
            let operands = if it.next()? == 1 {
                let pkt_count = read(11, it) as usize;
                it.batching(|it| parse_packet(it)).take(pkt_count).collect()
            } else {
                let payld_len = read(15, it) as usize;
                it.take(payld_len).batching(|it| parse_packet(it)).collect()
            };
            Payload::Operator { op, operands }
        }
    };
    Some(Packet { version, payload })
}

fn sum_versions(pkt: &Packet) -> u32 {
    pkt.version
        + match &pkt.payload {
            Payload::Literals(_) => 0,
            Payload::Operator { operands, .. } => operands.iter().map(sum_versions).sum(),
        }
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Option<Packet> {
    input
        .chars()
        .filter_map(|b| b.to_digit(16))
        .flat_map(|b| (0..4).rev().map(move |i| (b >> i) & 1))
        .batching(|it| parse_packet(it))
        .next()
}

#[aoc(day16, part1)]
fn part1(input: &Packet) -> u32 {
    sum_versions(input)
}

#[aoc(day16, part2)]
fn part2(input: &Packet) -> u32 {
    input.compute()
}

#[cfg(test)]
mod test {
    use super::gen;

    #[test]
    fn part1() {
        [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ]
        .into_iter()
        .for_each(|(input, expect)| assert_eq!(Some(expect), gen(input).as_ref().map(super::part1)))
    }

    #[test]
    fn part2() {
        [
            ("D2FE28", 2021),
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ]
        .into_iter()
        .for_each(|(input, expect)| {
            assert_eq!(Some(expect), gen(input).as_ref().map(super::part2))
        });
    }
}
