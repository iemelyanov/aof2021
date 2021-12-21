use std::{
    fmt::Display,
    io::{stdin, BufRead, BufReader},
};

struct BitSet {
    bits: Vec<u64>,
    size: usize,
}

impl BitSet {
    fn new(size: usize) -> Self {
        let mut n = size;
        if n < 64 {
            n = 64
        }
        n = n.next_power_of_two();
        Self {
            bits: vec![0 as u64; n],
            size: size,
        }
    }

    fn set(&mut self, i: usize) {
        self.bits[(i / 64)] |= 1 << (i % 64);
    }

    fn at(&self, i: usize) -> bool {
        self.bits[(i / 64)] & (1 << (i % 64)) > 0
    }

    fn len(&self) -> usize {
        self.size
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.len() {
            if self.at(i) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{}", s)
    }
}

fn hex_to_byte(h: u8) -> u8 {
    match h {
        b'0' => 0b0000,
        b'1' => 0b0001,
        b'2' => 0b0010,
        b'3' => 0b0011,
        b'4' => 0b0100,
        b'5' => 0b0101,
        b'6' => 0b0110,
        b'7' => 0b0111,
        b'8' => 0b1000,
        b'9' => 0b1001,
        b'A' => 0b1010,
        b'B' => 0b1011,
        b'C' => 0b1100,
        b'D' => 0b1101,
        b'E' => 0b1110,
        b'F' => 0b1111,
        _ => unreachable!(),
    }
}

fn encode(bits: &BitSet, from: usize, to: usize) -> u64 {
    if to - from > 64 {
        panic!("range grater than 64: {}", to - from);
    }

    let mut x = 0 as u64;
    for i in from..to {
        x <<= 1;
        x |= bits.at(i) as u64;
    }

    x
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal_val: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(version: u8, type_id: u8) -> Self {
        Self {
            version,
            type_id,
            literal_val: 0,
            sub_packets: Vec::new(),
        }
    }

    fn versions_sum(&self) -> usize {
        let mut sum = self.version as usize;

        for c in self.sub_packets.iter() {
            sum += c.versions_sum()
        }

        sum
    }

    fn eval(&self) -> u64 {
        match self.type_id {
            0 => {
                let mut sum = 0;
                for p in self.sub_packets.iter() {
                    sum += p.eval();
                }
                sum
            }
            1 => {
                let mut prod = 1;
                for p in self.sub_packets.iter() {
                    prod *= p.eval();
                }
                prod
            }
            2 => {
                let mut min = u64::MAX;
                for p in self.sub_packets.iter() {
                    min = min.min(p.eval());
                }
                min
            }
            3 => {
                let mut max = 0;
                for p in self.sub_packets.iter() {
                    max = max.max(p.eval());
                }
                max
            }
            4 => self.literal_val,
            5 => (self.sub_packets[0].eval() > self.sub_packets[1].eval()) as u64,
            6 => (self.sub_packets[0].eval() < self.sub_packets[1].eval()) as u64,
            7 => (self.sub_packets[0].eval() == self.sub_packets[1].eval()) as u64,
            _ => unreachable!(),
        }
    }
}

fn parse_header(bits: &BitSet, from: usize) -> (usize, u8 /* version */, u8 /* type_id */) {
    // println!("[parse header] at {}", from);
    let version = encode(bits, from, from + 3) as u8;
    let type_id = encode(bits, from + 3, from + 6) as u8;
    (6, version, type_id)
}

fn parse_literal(bits: &BitSet, from: usize) -> (usize, u64 /* value */) {
    // println!("[parse literal] at {}", from);
    let mut t = BitSet::new(64);
    let mut q = 1;
    let mut k = 0;
    for i in (from..bits.len()).step_by(5) {
        for j in i + 1..i + 5 {
            if bits.at(j) {
                t.set(k);
            }
            k += 1;
        }
        if !bits.at(i) {
            break;
        }
        q += 1
    }
    (k + q, encode(&t, 0, k))
}

#[derive(Debug)]
enum OperatorKind {
    TotalLengthInBits(usize),
    NumOfSubPackets(usize),
}

fn parse_operator(bits: &BitSet, from: usize) -> (usize, OperatorKind) {
    // println!("[parse operator] at {}", from);
    if bits.at(from) {
        (
            12,
            OperatorKind::NumOfSubPackets(encode(bits, from + 1, from + 12) as usize),
        )
    } else {
        (
            16,
            OperatorKind::TotalLengthInBits(encode(bits, from + 1, from + 16) as usize),
        )
    }
}

fn parse(bits: &BitSet, from: usize) -> (usize, Box<Packet>) {
    // println!("[parse] at {}", from);
    let (mut total_bits, version, type_id) = parse_header(bits, from);
    let mut packet = Box::new(Packet::new(version, type_id));
    let mut from = from + total_bits;
    if type_id == 4 {
        let (nbits, value) = parse_literal(bits, from);
        packet.literal_val = value;
        return (total_bits + nbits, packet);
    } else {
        let (nbits, kind) = parse_operator(bits, from);
        total_bits += nbits;
        from += nbits;
        match kind {
            OperatorKind::NumOfSubPackets(n) => {
                for _ in 0..n {
                    let (nbits, new_packet) = parse(bits, from);
                    total_bits += nbits;
                    from += nbits;
                    packet.sub_packets.push(*new_packet);
                }
            }
            OperatorKind::TotalLengthInBits(mut n) => {
                while n > 0 {
                    let (nbits, new_packet) = parse(bits, from);
                    total_bits += nbits;
                    from += nbits;
                    packet.sub_packets.push(*new_packet);
                    n -= nbits;
                }
            }
        }
    }

    (total_bits, packet)
}

fn main() {
    let mut buf = String::new();
    if let Err(why) = BufReader::new(stdin()).read_line(&mut buf) {
        panic!("{}", why);
    }
    let mut bits = BitSet::new(buf.len() * 4);
    let mut k = 0;
    for b in buf.as_bytes() {
        let z = hex_to_byte(*b);
        let mut x = 0b1000;
        while x > 0 {
            if x & z > 0 {
                bits.set(k);
            }
            x >>= 1;
            k += 1;
        }
    }
    let (_, packet) = parse(&bits, 0);
    // println!("{:?}", packet);
    println!("{}", packet.versions_sum());
    println!("{}", packet.eval());
}
