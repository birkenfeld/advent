use advtools::input::input_string;

const COLS: usize = 25;
const SIZE: usize = COLS * 6;

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn main() {
    let img = input_string();
    let img = img.trim().as_bytes();

    // The chunks iterator is pretty helpful for this.
    let result = img.chunks(SIZE).map(|layer| {
        let zeros = layer.iter().filter(|&&px| px == BLACK).count();
        let ones = layer.iter().filter(|&&px| px == WHITE).count();
        // The number of twos must be the remaining...
        (zeros, ones*(layer.len() - zeros - ones))
    }).min().unwrap();
    advtools::print("Ones*twos in min-zero layer", result.1);

    advtools::print("Message", "");
    for i in 0..SIZE {
        // Find the first non-transparent pixel when going through the layers.
        let px = img[i..].iter().step_by(SIZE).find(|&&px| px != TRANSPARENT);
        print!("{}", if px == Some(&WHITE) { '█' } else { ' ' });
        if i % COLS == COLS-1 { println!(); }
    }
}
