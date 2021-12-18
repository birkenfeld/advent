use advtools::input::input_string;

#[derive(Debug)]
enum Pkt {
    Literal(u64),
    Operator(u16, Vec<Pkt>),
}

impl Pkt {
    fn evaluate(&self) -> u64 {
        match self {
            Pkt::Literal(n) => *n,
            Pkt::Operator(0, sub) => sub.iter().map(|p| p.evaluate()).sum(),
            Pkt::Operator(1, sub) => sub.iter().map(|p| p.evaluate()).product(),
            Pkt::Operator(2, sub) => sub.iter().map(|p| p.evaluate()).min().unwrap(),
            Pkt::Operator(3, sub) => sub.iter().map(|p| p.evaluate()).max().unwrap(),
            Pkt::Operator(5, sub) => (sub[0].evaluate() > sub[1].evaluate()) as _,
            Pkt::Operator(6, sub) => (sub[0].evaluate() < sub[1].evaluate()) as _,
            Pkt::Operator(7, sub) => (sub[0].evaluate() == sub[1].evaluate()) as _,
            _ => unreachable!()
        }
    }
}

struct Message {
    data: Vec<u8>,
    cur: usize,
    versions: u16
}

impl Message {
    fn push(&mut self, ch: char) {
        let n = if ch > '9' { (ch as u8) - 55 } else { (ch as u8) - b'0' };
        self.data.push(n >> 3);
        self.data.push((n >> 2) & 1);
        self.data.push((n >> 1) & 1);
        self.data.push(n & 1);
    }

    fn consume(&mut self, n: usize) -> u16 {
        let mut result = 0;
        for i in 0..n {
            result = (result << 1) | self.data[self.cur + i] as u16;
        }
        self.cur += n;
        result
    }

    fn packet(&mut self) -> Pkt {
        self.versions += self.consume(3);
        let pktype = self.consume(3);
        if pktype == 4 {  // Literal value
            let mut data = vec![];
            loop {
                let chunk = self.consume(5);
                data.push(chunk as u8 & 0xf);
                if chunk & 0x10 == 0 {
                    break;
                }
            }
            let num = data.into_iter().fold(0, |acc, nib| (acc << 4) | nib as u64);
            Pkt::Literal(num)
        } else {
            let mut subpkts = vec![];
            let lentype = self.consume(1);
            if lentype == 0 {
                let length = self.consume(15) as usize;
                let end_at = self.cur + length;
                while self.cur < end_at {
                    subpkts.push(self.packet());
                }
            } else {
                let npkts = self.consume(11);
                for _ in 0..npkts {
                    subpkts.push(self.packet());
                }
            }
            Pkt::Operator(pktype, subpkts)
        }
    }
}

fn main() {
    let mut message = Message { data: vec![], cur: 0, versions: 0 };
    input_string().trim().chars().for_each(|ch| message.push(ch));

    let packet = message.packet();
    advtools::verify("Sum of versions", message.versions, 886);
    advtools::verify("Packet value", packet.evaluate(), 184487454837u64);
}
