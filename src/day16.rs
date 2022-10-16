use yaah::*;
use itertools::Itertools;

#[derive(Debug)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
pub enum Payload {
    Literals(u32),
    Operation {
        operator: Operator,
        operands: Vec<Packet>,
    },
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    payload: Payload,
}
impl Packet {
    fn compute(&self) -> u32 {
        match &self.payload {
            Payload::Literals(v) => *v,
            Payload::Operation {
                operator: op,
                operands,
            } => {
                let operands = operands.iter().map(|op| op.compute()).collect_vec();
                match (op, operands.as_slice()) {
                    (Operator::Sum, _) => operands.into_iter().sum(),
                    (Operator::Product, _) => operands.into_iter().product(),
                    (Operator::Minimum, _) => operands.into_iter().min().expect("No operands!"),
                    (Operator::Maximum, _) => operands.into_iter().max().expect("No operands!"),
                    (Operator::GreaterThan, [a, b]) => (a > b) as u32,
                    (Operator::LessThan, [a, b]) => (a < b) as u32,
                    (Operator::EqualTo, [a, b]) => (a == b) as u32,
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn read(bits: usize, stream: &mut (impl Iterator<Item = u32> + ?Sized)) -> Option<u32> {
    let mut cnt = 0;
    let v = stream.take(bits).fold(0, |acc, b| {
        cnt += 1;
        acc << 1 | b
    });
    (cnt == bits).then(|| v)
}

fn parse_packet(it: &mut (dyn Iterator<Item = u32>)) -> Option<Packet> {
    let version = read(3, it)?;
    let type_id = read(3, it)?;
    let payload = match type_id {
        4 => {
            let mut literal = 0;
            let mut it = it.batching(|it| read(5, it));
            loop {
                let b = it.next()?;
                literal = literal << 4 | (b & 0xF);
                if (b & 0x10) == 0 {
                    break;
                }
            }
            Payload::Literals(literal)
        }
        op_code => {
            let op = match op_code {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::GreaterThan,
                6 => Operator::LessThan,
                7 => Operator::EqualTo,
                _ => unreachable!(),
            };
            let operands = if it.next()? == 1 {
                let pkt_count = read(11, it)? as usize;
                let ops: Vec<_> = it.batching(|it| parse_packet(it)).take(pkt_count).collect();
                (ops.len() == pkt_count).then(|| ops)
            } else {
                let pld_len = read(15, it)? as usize;
                let mut cnt = 0;
                let ops = it
                    .take(pld_len)
                    .inspect(|_| cnt += 1)
                    .batching(|it| parse_packet(it))
                    .collect();
                (cnt == pld_len).then(|| ops)
            }?;
            Payload::Operation {
                operator: op,
                operands,
            }
        }
    };
    Some(Packet { version, payload })
}

fn sum_versions(pkt: &Packet) -> u32 {
    pkt.version
        + match &pkt.payload {
            Payload::Literals(_) => 0,
            Payload::Operation { operands, .. } => operands.iter().map(sum_versions).sum(),
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
