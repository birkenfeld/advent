extern crate advtools;
use advtools::prelude::*;

fn pixel(ch: char) -> u8 {
    match ch {
        '.' => 0,
        '#' => 1,
        _ => unreachable!()
    }
}

fn f2x2(mut v: Vec<u8>) -> Vec<u8> {
    v.swap(0, 1);
    v.swap(2, 3);
    v
}

fn r2x2(mut v: Vec<u8>) -> Vec<u8> {
    v.swap(0, 1);
    v.swap(2, 3);
    v.swap(0, 3);
    v
}

fn f3x3(mut v: Vec<u8>) -> Vec<u8> {
    v.swap(0, 2);
    v.swap(3, 5);
    v.swap(6, 8);
    v
}

fn r3x3(mut v: Vec<u8>) -> Vec<u8> {
    v.swap(0, 2);
    v.swap(1, 5);
    v.swap(0, 8);
    v.swap(1, 3);
    v.swap(3, 7);
    v.swap(0, 6);
    v
}

fn advance(step: u32, state: Vec<Vec<u8>>, repls: &HashMap<Vec<u8>, Vec<u8>>) -> Vec<Vec<u8>> {
    match step % 3 {
        0 => {
            let mut new = vec![vec![]; state.len()*2];
            for j in (0..state[0].len()).step(9) {
                for (i, row) in state.iter().enumerate() {
                    let repl = &repls[&row[j..j+9]];
                    new[2*i  ].extend(vec![repl[0], repl[1], repl[4], repl[5],
                                           repl[2], repl[3], repl[6], repl[7]]);
                    new[2*i+1].extend(vec![repl[8], repl[9], repl[12], repl[13],
                                           repl[10], repl[11], repl[14], repl[15]]);
                }
            }
            new
        }
        1 => {
            let mut new = vec![vec![]; state.len()*3/2];
            for j in (0..state[0].len()).step(8) {
                for (i, (row1, row2)) in state.iter().tuples().enumerate() {
                    let repl1 = &repls[&row1[j..j+4]];
                    let repl2 = &repls[&row1[j+4..j+8]];
                    let repl3 = &repls[&row2[j..j+4]];
                    let repl4 = &repls[&row2[j+4..j+8]];
                    new[3*i  ].extend(vec![repl1[0], repl1[1], repl1[3], repl1[4],
                                           repl1[2], repl2[0], repl1[5], repl2[3],
                                           repl2[1], repl2[2], repl2[4], repl2[5]]);
                    new[3*i+1].extend(vec![repl1[6], repl1[7], repl3[0], repl3[1],
                                           repl1[8], repl2[6], repl3[2], repl4[0],
                                           repl2[7], repl2[8], repl4[1], repl4[2]]);
                    new[3*i+2].extend(vec![repl3[3], repl3[4], repl3[6], repl3[7],
                                           repl3[5], repl4[3], repl3[8], repl4[6],
                                           repl4[4], repl4[5], repl4[7], repl4[8]]);
                }
            }
            new
        }
        2 => {
            let mut new = vec![vec![]; state.len()];
            for j in (0..state[0].len()).step(4) {
                for (i, row) in state.iter().enumerate() {
                    new[i].extend(repls[&row[j..j+4]].clone());
                }
            }
            new
        }
        _ => unreachable!()
    }
}

fn main() {
    let mut state: Vec<Vec<u8>> = vec![vec![0, 1, 0,
                                            0, 0, 1,
                                            1, 1, 1]];
    let mut repls = HashMap::new();
    for line in iter_input::<Vec<String>>() {
        let pattern = line[0].chars().filter(|&c| c != '/').map(pixel).collect_vec();
        let repl = line[2].chars().filter(|&c| c != '/').map(pixel).collect_vec();
        if line[0].len() == 5 {
            let flipped = f2x2(pattern.clone());
            repls.insert(r2x2(flipped.clone()), repl.clone());
            repls.insert(r2x2(r2x2(flipped.clone())), repl.clone());
            repls.insert(r2x2(r2x2(r2x2(flipped.clone()))), repl.clone());
            repls.insert(flipped, repl.clone());
            repls.insert(r2x2(pattern.clone()), repl.clone());
            repls.insert(r2x2(r2x2(pattern.clone())), repl.clone());
            repls.insert(r2x2(r2x2(r2x2(pattern.clone()))), repl.clone());
            repls.insert(pattern, repl);
        } else {
            let flipped = f3x3(pattern.clone());
            repls.insert(r3x3(flipped.clone()), repl.clone());
            repls.insert(r3x3(r3x3(flipped.clone())), repl.clone());
            repls.insert(r3x3(r3x3(r3x3(flipped.clone()))), repl.clone());
            repls.insert(flipped, repl.clone());
            repls.insert(r3x3(pattern.clone()), repl.clone());
            repls.insert(r3x3(r3x3(pattern.clone())), repl.clone());
            repls.insert(r3x3(r3x3(r3x3(pattern.clone()))), repl.clone());
            repls.insert(pattern, repl);
        }
    }

    for n in 0..5 {
        state = advance(n, state, &repls);
    }
    println!("Lights on after 5: {}", state.iter().map(
        |row| row.iter().map(|&v| v as u32).sum::<u32>()).sum::<u32>());

    for n in 5..18 {
        state = advance(n, state, &repls);
    }
    println!("Lights on after 18: {}", state.iter().map(
        |row| row.iter().map(|&v| v as u32).sum::<u32>()).sum::<u32>());
}
