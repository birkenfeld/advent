use advtools::input;

// Calculate the area of the polygon defined by the given instructions.
fn area(instructions: Vec<((isize, isize), isize)>) -> isize {
    let mut poly: Vec<(isize, isize)> = vec![(0, 0)];
    let mut tiles = 0;

    // Create polygon coordinates from instructions.  We assume the points to be
    // in the center of each tile, so there is no need to correct coordinates
    // for half-tile offsets, but we need to compensate the total area below.
    for (dir, num) in instructions {
        let last = poly.last().unwrap();
        let new = (last.0 + dir.0 * num, last.1 + dir.1 * num);
        poly.push(new);
        tiles += num;
    }

    // Calculate inner area.
    let area = poly.iter().zip(poly.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| x1*y2 - x2*y1).sum::<isize>() / 2;

    // Now compensate for the parts of the border that are outside the
    // polygon.  This is half a tile for every straight tile, and an
    // additional +/- 1/4 tile for each corner, which all cancel except
    // for 4 outside corners.
    area + tiles/2 + 1
}

const REGEX: &str = r"(.) (\d+) \(#(.*)(.)\)";

fn main() {
    // Parse the input for both parts.
    let mut part1 = Vec::new();
    let mut part2 = Vec::new();
    for (dir1, num1, num2, dir2) in input::rx_lines::<(char, isize, &str, char)>(REGEX) {
        part1.push((match dir1 {
            'R' => (1, 0), 'D' => (0, 1), 'L' => (-1, 0), 'U' => (0, -1), _ => unreachable!()
        }, num1));
        part2.push((match dir2 {
            '0' => (1, 0), '1' => (0, 1), '2' => (-1, 0), '3' => (0, -1), _ => unreachable!()
        }, isize::from_str_radix(num2, 16).unwrap()));
    }

    advtools::verify("Lava capacity", area(part1), 31171);
    advtools::verify("With real instructions", area(part2), 131431655002266_u64);
}
