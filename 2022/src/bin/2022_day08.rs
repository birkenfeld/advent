use advtools::input;
use advtools::grid::Grid;

fn main() {
    let size = Grid::new(input::lines().map(|line| {
        line.bytes().map(|height| height - b'0')
    }));
    let (nx, ny) = (size.width(), size.height());

    let mut visible = 2 * (nx + ny - 2);  // already include edge trees
    let mut max_view = 0;

    for x in 1..size.width()-1 {
        for y in 1..size.height()-1 {
            let cur = size[(x, y)];

            if (0..x).all(|xi| size[(xi, y)] < cur) ||
                (x+1..nx).all(|xi| size[(xi, y)] < cur) ||
                (0..y).all(|yi| size[(x, yi)] < cur) ||
                (y+1..ny).all(|yi| size[(x, yi)] < cur)
            {
                visible += 1;
            }

            let views = (
                (0..x).rev().position(|xi| size[(xi, y)] >= cur).map_or(x, |n| n + 1),
                (x+1..nx).position(|xi| size[(xi, y)] >= cur).map_or(nx - x - 1, |n| n + 1),
                (0..y).rev().position(|yi| size[(x, yi)] >= cur).map_or(y, |n| n + 1),
                (y+1..ny).position(|yi| size[(x, yi)] >= cur).map_or(ny - y - 1, |n| n + 1)
            );

            max_view = max_view.max(views.0 * views.1 * views.2 * views.3);
        }
    }

    advtools::verify("Visible trees", visible, 1820);
    advtools::verify("Best view", max_view, 385112);
}
