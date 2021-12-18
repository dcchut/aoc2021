use crate::{FromProblemInput, ProblemInput, Solution};

pub struct Q16;

#[derive(Debug, Copy, Clone)]
pub struct Packet<'input> {
    bits: &'input str,
}

impl<'input> Packet<'input> {
    fn split(&mut self, n: usize) -> Self {
        let (l, r) = self.bits.split_at(n);
        self.bits = r;
        Packet { bits: l }
    }

    fn take(&mut self, n: usize) -> i64 {
        i64::from_str_radix(self.take_raw(n), 2).unwrap()
    }

    fn take_raw<'a>(&'a mut self, n: usize) -> &'input str {
        let (l, r) = self.bits.split_at(n);
        self.bits = r;
        l
    }

    fn peek(&self) -> Option<char> {
        self.bits.chars().next()
    }

    fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
}

#[derive(Debug, Clone)]
enum ParsedPacketKind {
    Literal(i64),
    Operator(Vec<ParsedPacket>),
}

#[derive(Debug, Clone)]
struct ParsedPacket {
    version: i64,
    typ: i64,
    kind: ParsedPacketKind,
}

fn parse_literal(packet: &mut Packet<'_>) -> i64 {
    let mut bits = Vec::new();

    loop {
        let group_terminator = packet.peek().unwrap() == '0';
        packet.take(1);
        let raw = packet.take_raw(4);
        bits.push(raw);
        if group_terminator {
            break;
        }
    }

    i64::from_str_radix(&bits.into_iter().collect::<String>(), 2).unwrap()
}

fn parse(packet: &mut Packet<'_>) -> ParsedPacket {
    let version = packet.take(3);
    let typ = packet.take(3);

    // Literal packets
    if typ == 4 {
        let value = parse_literal(packet);
        return ParsedPacket {
            version,
            typ,
            kind: ParsedPacketKind::Literal(value),
        };
    }

    // Operator packets
    let mut parsed_sub_packets = Vec::new();
    let length_type_id = packet.take(1);

    if length_type_id == 0 {
        let packet_length = packet.take(15) as usize;

        let mut sub_packets = packet.split(packet_length);
        while !sub_packets.is_empty() {
            parsed_sub_packets.push(parse(&mut sub_packets));
        }
    } else {
        let sub_packets = packet.take(11) as usize;
        for _ in 0..sub_packets {
            parsed_sub_packets.push(parse(packet));
        }
    }

    ParsedPacket {
        version,
        typ,
        kind: ParsedPacketKind::Operator(parsed_sub_packets),
    }
}

#[derive(Debug, Clone)]
pub struct HexBits(String);

impl FromProblemInput<'_> for HexBits {
    fn from(lines: &ProblemInput) -> Self {
        let bits = lines.lines[0]
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .map(|c| format!("{:0>4b}", c))
            .collect();
        Self(bits)
    }
}

impl HexBits {
    fn parse(&self) -> ParsedPacket {
        let mut packet = Packet {
            bits: self.0.as_str(),
        };
        parse(&mut packet)
    }
}

fn evaluate(p: &ParsedPacket) -> (i64, i64) {
    let mut version = p.version;

    let value = match &p.kind {
        ParsedPacketKind::Literal(literal) => *literal,
        ParsedPacketKind::Operator(sub_packets) => {
            let mut values = Vec::new();
            for packet in sub_packets {
                let (sub_version, sub_value) = evaluate(packet);
                version += sub_version;
                values.push(sub_value);
            }

            match p.typ {
                0 => values.into_iter().sum::<i64>(),
                1 => values.into_iter().product::<i64>(),
                2 => values.into_iter().min().unwrap(),
                3 => values.into_iter().max().unwrap(),
                5 => (values[0] > values[1]) as i64,
                6 => (values[0] < values[1]) as i64,
                7 => (values[0] == values[1]) as i64,
                _ => panic!("unrecognized type {}", p.typ),
            }
        }
    };

    (version, value)
}

impl Solution for Q16 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let hex_bits: HexBits = lines.parse();
        let parsed = hex_bits.parse();
        evaluate(&parsed).0.to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let hex_bits: HexBits = lines.parse();
        let parsed = hex_bits.parse();
        evaluate(&parsed).1.to_string()
    }
}
