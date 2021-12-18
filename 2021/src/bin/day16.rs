use advtools::input;

enum Pkt {
    Literal(u16, u64),
    Operator(u16, u16, Vec<Pkt>),
}

fn consume(bits: &mut dyn Iterator<Item=u16>, n: usize) -> u16 {
    (0..n).fold(0, |res, _| (res << 1) | bits.next().unwrap())
}

impl Pkt {
    fn parse(bits: &mut dyn Iterator<Item=u16>) -> Self {
        let version = consume(bits, 3);
        let pktype = consume(bits, 3);
        if pktype == 4 {  // Literal value
            let mut data = vec![];
            loop {
                let chunk = consume(bits, 5);
                data.push(chunk as u8 & 0xf);
                if chunk & 0x10 == 0 {
                    break;
                }
            }
            let num = data.into_iter().fold(0, |acc, nib| (acc << 4) | nib as u64);
            Pkt::Literal(version, num)
        } else {
            let mut subpkts = vec![];
            let lentype = consume(bits, 1);
            if lentype == 0 {
                let length = consume(bits, 15) as usize;
                let mut new_bits = bits.take(length).peekable();
                while new_bits.peek().is_some() {
                    subpkts.push(Pkt::parse(&mut new_bits));
                }
            } else {
                let npkts = consume(bits, 11);
                for _ in 0..npkts {
                    subpkts.push(Pkt::parse(bits));
                }
            }
            Pkt::Operator(version, pktype, subpkts)
        }
    }

    fn version_sum(&self) -> u16 {
        match self {
            Pkt::Literal(v, _) => *v,
            Pkt::Operator(v, _, sub) => *v + sub.iter().map(Pkt::version_sum).sum::<u16>(),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Pkt::Literal(_, n) => *n,
            Pkt::Operator(_, 0, sub) => sub.iter().map(|p| p.evaluate()).sum(),
            Pkt::Operator(_, 1, sub) => sub.iter().map(|p| p.evaluate()).product(),
            Pkt::Operator(_, 2, sub) => sub.iter().map(|p| p.evaluate()).min().unwrap(),
            Pkt::Operator(_, 3, sub) => sub.iter().map(|p| p.evaluate()).max().unwrap(),
            Pkt::Operator(_, 5, sub) => (sub[0].evaluate() > sub[1].evaluate()) as _,
            Pkt::Operator(_, 6, sub) => (sub[0].evaluate() < sub[1].evaluate()) as _,
            Pkt::Operator(_, 7, sub) => (sub[0].evaluate() == sub[1].evaluate()) as _,
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut bits = input::chars()
        .map(|ch| ch.to_digit(16).unwrap() as u16)
        .flat_map(|n| [n >> 3, (n >> 2) & 1, (n >> 1) & 1, n & 1]);
    let packet = Pkt::parse(&mut bits);

    advtools::verify("Sum of versions", packet.version_sum(), 886);
    advtools::verify("Packet value", packet.evaluate(), 184487454837u64);
}
