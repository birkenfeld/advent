const SIZE: usize = 10001;

enum State { A, B, C, D, E, F }
use self::State::*;

fn main() {
    let mut tape = vec![false; SIZE];
    let mut state = A;
    let mut ptr = SIZE/2;

    for _ in 0..12586542 {
        state = match (state, tape[ptr]) {
            (A, false) => { tape[ptr] = true;  ptr += 1; B },
            (A, true)  => { tape[ptr] = false; ptr -= 1; B },
            (B, false) => {                    ptr += 1; C },
            (B, true)  => {                    ptr -= 1; B },
            (C, false) => { tape[ptr] = true;  ptr += 1; D },
            (C, true)  => { tape[ptr] = false; ptr -= 1; A },
            (D, false) => { tape[ptr] = true;  ptr -= 1; E },
            (D, true)  => {                    ptr -= 1; F },
            (E, false) => { tape[ptr] = true;  ptr -= 1; A },
            (E, true)  => { tape[ptr] = false; ptr -= 1; D },
            (F, false) => { tape[ptr] = true;  ptr += 1; A },
            (F, true)  => {                    ptr -= 1; E },
        }
    }

    let cksum = tape.iter().map(|&v| v as u32).sum::<u32>();
    println!("Diagnostic checksum: {}", cksum);
}
