pub fn knot_process(input: &[u8], n: u32) -> Vec<u8> {
    let mut marks = (0..).take(256).collect::<Vec<_>>();
    let mut skip = 0;
    let mut total_rot = 0;
    for _ in 0..n {
        for &length in input {
            marks[..length as usize].reverse();
            let pos = length.wrapping_add(skip);
            marks.rotate_left(pos as usize);
            total_rot += pos as usize;
            skip += 1;
        }
    }
    marks.rotate_right(total_rot % 256);
    marks
}

pub fn knot_hash(input: &str) -> Vec<u8> {
    let mut input = input.as_bytes().to_vec();
    input.extend([17, 31, 73, 47, 23]);
    let sparse = knot_process(&input, 64);
    sparse.chunks(16).map(|v| v.iter().fold(0, |a, b| a^b)).collect()
}
