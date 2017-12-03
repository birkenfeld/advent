extern crate advtools;

const INPUT: u32 = 312051;

fn next_pos((x, y): (i32, i32)) -> (i32, i32) {
    if x > 0 && (x.abs() > y.abs()) {
        (x, y + 1)
    } else if x < 0 && (x.abs() > y.abs() || x == -y) {
        (x, y - 1)
    } else if y > 0 && (y.abs() > x.abs() || x == y) {
        (x - 1, y)
    } else if y < 0 && (y.abs() > x.abs() || x == y) {
        (x + 1, y)
    } else {
        (x + 1, y)
    }
}

fn main() {
    let mut pos = (0, 0);
    for _ in 1..INPUT {
        pos = next_pos(pos);
    }
    println!("Distance for {}: {}", INPUT, pos.0.abs() + pos.1.abs());

    let mut pos = (1, 0);
    let mut arr = vec![vec![0; 200]; 200];
    arr[100][100] = 1;
    let value = loop {
        let (ix, iy) = ((pos.0 + 100) as usize, (pos.1 + 100) as usize);
        let write = arr[ix+1][iy] + arr[ix-1][iy] + arr[ix][iy+1] + arr[ix][iy-1] +
            arr[ix+1][iy+1] + arr[ix+1][iy-1] + arr[ix-1][iy+1] + arr[ix-1][iy-1];
        if write > INPUT {
            break write;
        }
        arr[ix][iy] = write;
        pos = next_pos(pos);
    };
    println!("Value written: {}", value);
}
