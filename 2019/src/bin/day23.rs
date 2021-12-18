use advtools::prelude::Itertools;
use advtools::input;
use advent19::{Machine, IO};

const N: i64 = 50;

fn main() {
    let code = Machine::parse(input::string());

    let mut machines = (0..N).map(|addr| {
        // Create machines with initial input (the first -1 is to avoid sending
        // a useless NAT message below).
        Machine::new(&code).with_input(addr).with_input(-1)
    }).collect_vec();

    let mut nat = (0, 0);
    let mut last_nat_sent = (0, 0);
    let mut packets = Vec::new();

    loop {
        let mut waiting = 0;
        // Run each NIC until it either needs input, or produces a packet.
        for machine in &mut machines {
            match machine.run() {
                IO::Input => {
                    machine.push_input(-1);
                    waiting += 1;
                }
                IO::Output(target_addr) => {
                    let (x, y) = machine.next_tuple().unwrap();
                    packets.push((target_addr as usize, x, y));
                }
                _ => unreachable!()
            }
        }
        // If all machines are waiting, send a NAT packet to machine 0.
        if waiting == machines.len() {
            if nat == last_nat_sent {
                advtools::verify("NAT-Y sent twice in a row", nat.1, 12415);
                return;
            }
            machines[0].push_input(nat.0);
            machines[0].push_input(nat.1);
            last_nat_sent = nat;
        }
        // Handle the queue of emitted packets and distribute it to
        // the receiver NICs.
        for (addr, x, y) in packets.drain(..) {
            if addr == 255 {
                if nat == (0, 0) {
                    advtools::verify("First NAT-Y received", y, 17541);
                }
                nat = (x, y);
            } else {
                machines[addr].push_input(x);
                machines[addr].push_input(y);
            }
        }
    }
}
