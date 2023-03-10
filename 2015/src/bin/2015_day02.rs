use advtools::input;

fn main() {
    let (total_paper, total_ribbon) = input::rx_lines(r"(\d+)x(\d+)x(\d+)").fold(
        (0, 0), |(paper, ribbon), mut dimensions: [u32; 3]| {
            dimensions.sort_unstable();
            let [l, w, h] = dimensions;
            (paper + 2 * (l*w + w*h + h*l) + l*w, ribbon + l*w*h + 2 * (l + w))
        });
    advtools::verify("Paper", total_paper, 1588178);
    advtools::verify("Ribbon", total_ribbon, 3783758);
}
