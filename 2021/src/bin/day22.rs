use advtools::prelude::{HashMap, HashSet, Itertools};
use advtools::input;

const FORMAT: &str = r"(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)";

fn switch(coords: &[(bool, (i32, i32), (i32, i32), (i32, i32))]) -> u64 {
    // Collect all X/Y/Z coordinates that occur as cube edges.
    let mut x_coords = HashSet::new();
    let mut y_coords = HashSet::new();
    let mut z_coords = HashSet::new();
    for &(_, (x1, x2), (y1, y2), (z1, z2)) in coords {
        x_coords.insert(x1); x_coords.insert(x2);
        y_coords.insert(y1); y_coords.insert(y2);
        z_coords.insert(z1); z_coords.insert(z2);
    }
    let x_coords = x_coords.into_iter().sorted().collect_vec();
    let y_coords = y_coords.into_iter().sorted().collect_vec();
    let z_coords = z_coords.into_iter().sorted().collect_vec();

    // Map the spans between edges to indices in a big 3-D grid.
    let x_map: HashMap<_, _> = x_coords.iter().enumerate().map(|(i, x)| (x, i)).collect();
    let y_map: HashMap<_, _> = y_coords.iter().enumerate().map(|(i, x)| (x, i)).collect();
    let z_map: HashMap<_, _> = z_coords.iter().enumerate().map(|(i, x)| (x, i)).collect();
    let nx = x_map.len() - 1;
    let ny = y_map.len() - 1;
    let nz = z_map.len() - 1;

    let mut grid = vec![false; nx * ny * nz];

    // Switch individual regions on or off.
    for &(on, (x1, x2), (y1, y2), (z1, z2)) in coords {
        for xi in x_map[&x1]..x_map[&x2] {
            for yi in y_map[&y1]..y_map[&y2] {
                for zi in z_map[&z1]..z_map[&z2] {
                    grid[ny*nz*xi + nz*yi + zi] = on;
                }
            }
        }
    }

    // Since every pixel in our grid has different physical size in cubes,
    // reconstruct the number of actual cubes using the initial coordinates.
    let mut count = 0u64;
    for xi in 0..nx {
        for yi in 0..ny {
            for zi in 0..nz {
                if grid[ny*nz*xi + nz*yi + zi] {
                    count += (x_coords[xi+1] - x_coords[xi]) as u64 *
                        (y_coords[yi+1] - y_coords[yi]) as u64 *
                        (z_coords[zi+1] - z_coords[zi]) as u64;
                }
            }
        }
    }

    count
}

fn main() {
    let mut part1 = 0;
    let coords = input::rx_lines(FORMAT).map(|line| {
        let (onoff, [x1, x2, y1, y2, z1, z2]): (&str, [i32; 6]) = line;
        if -50 <= x1 && x1 <= 50 { part1 += 1; }
        (onoff == "on", (x1, x2 + 1), (y1, y2 + 1), (z1, z2 + 1))
    }).collect_vec();

    advtools::verify("Initialization", switch(&coords[..part1]), 589411);
    advtools::verify("Full", switch(&coords), 1130514303649907_u64);
}
