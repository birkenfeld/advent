extern crate advtools;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Source {
    Const(u16),
    Wire(u16),
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Const(Source),
    And(Source, Source),
    Or(Source, Source),
    Not(Source),
    Lshift(Source, Source),
    Rshift(Source, Source)
}

struct Circuit {
    wires: HashMap<u16, Element>,
    values: HashMap<u16, u16>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            wires: HashMap::new(),
            values: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.values.clear();
    }

    fn connect(&mut self, id: u16, el: Element) {
        if self.wires.insert(id, el).is_some() {
            panic!("duplicate wire: {}", id);
        }
    }

    fn reconnect(&mut self, id: u16, el: Element) {
        self.wires.insert(id, el);
    }

    fn get(&mut self, source: &Source) -> u16 {
        match *source {
            Source::Const(v) => v,
            Source::Wire(id) => self.get_value(id)
        }
    }

    fn get_value(&mut self, id: u16) -> u16 {
        if self.values.contains_key(&id) {
            self.values[&id]
        } else {
            let element = self.wires[&id];
            let value = match element {
                Element::Const(ref s) => self.get(s),
                Element::Not(ref s) => !self.get(s),
                Element::And(ref s1, ref s2) => self.get(s1) & self.get(s2),
                Element::Or(ref s1, ref s2) => self.get(s1) | self.get(s2),
                Element::Lshift(ref s1, ref s2) => self.get(s1) << self.get(s2),
                Element::Rshift(ref s1, ref s2) => self.get(s1) >> self.get(s2),
            };
            self.values.insert(id, value);
            value
        }
    }
}

fn wire_id(name: &str) -> u16 {
    name.chars().enumerate().map(|(i, c)| (c as u16) << (8 * i)).sum()
}

fn parse_source(src: &str) -> Source {
    match src.parse::<u16>() {
        Ok(num) => Source::Const(num),
        _ => Source::Wire(wire_id(src)),
    }
}

fn parse_connection(tok: Vec<&str>) -> (u16, Element) {
    match tok[1] {
        "->" => (wire_id(tok[2]), Element::Const(parse_source(tok[0]))),
        "AND" => (wire_id(tok[4]), Element::And(parse_source(tok[0]),
                                                parse_source(tok[2]))),
        "OR" => (wire_id(tok[4]), Element::Or(parse_source(tok[0]),
                                              parse_source(tok[2]))),
        "LSHIFT" => (wire_id(tok[4]), Element::Lshift(parse_source(tok[0]),
                                                      parse_source(tok[2]))),
        "RSHIFT" => (wire_id(tok[4]), Element::Rshift(parse_source(tok[0]),
                                                      parse_source(tok[2]))),
        v if tok[0] == "NOT" => (wire_id(tok[3]), Element::Not(parse_source(v))),
        _ => panic!("unrecognized line: {:?}", tok.join(" "))
    }
}

fn main() {
    let mut circuit = Circuit::new();
    for line in advtools::iter_input::<String>() {
        let parts = line.split_whitespace().collect();
        let (id, el) = parse_connection(parts);
        circuit.connect(id, el);
    }
    let signal_a = circuit.get_value(wire_id("a"));
    println!("Signal at wire a: {}", signal_a);
    circuit.reset();
    circuit.reconnect(wire_id("b"), Element::Const(Source::Const(signal_a)));
    println!("Signal at wire a after modification: {}", circuit.get_value(wire_id("a")));
}
