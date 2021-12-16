use aoc_runner_derive::*;
use itertools::Itertools;

#[derive(Debug)]
enum Payload {
    Literals(Vec<u32>),
    Operator { operands: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
    version: u32,
    payload: Payload,
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
            let mut literals = Vec::new();
            while let Some(cont) = it.next() {
                let val = read(4, it);
                literals.push(val);
                if cont == 0 {
                    break;
                }
            }
            Payload::Literals(literals)
        }
        _op_code => {
            let operands = if it.next()? == 1 {
                let pkt_count = read(11, it) as usize;
                it.batching(|it| parse_packet(it)).take(pkt_count).collect()
            } else {
                let payld_len = read(15, it) as usize;
                it.take(payld_len).batching(|it| parse_packet(it)).collect()
            };
            // parse operands
            Payload::Operator { operands }
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

#[aoc(day16, part1)]
fn part1(input: &str) -> Option<u32> {
    input
        .chars()
        .filter_map(|b| b.to_digit(16))
        .flat_map(|b| (0..4).rev().map(move |i| (b >> i) & 1))
        .batching(|it| parse_packet(it))
        .next()
        .map(|pkt| sum_versions(&pkt))
}

#[cfg(test)]
mod test {
    const EXAMPLE1: &str = r"D2FE28";
    const EXAMPLE2: &str = r"38006F45291200";
    const EXAMPLE3: &str = r"EE00D40C823060";

    #[test]
    fn part1() {
        assert_eq!(Some(16), super::part1("8A004A801A8002F478"));
        assert_eq!(Some(12), super::part1("620080001611562C8802118E34"));
        assert_eq!(Some(23), super::part1("C0015000016115A2E0802F182340"));
        assert_eq!(Some(31), super::part1("A0016C880162017C3686B18A3D4780"));
    }
}
