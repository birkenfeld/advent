use advtools::input::iter_input_regex;

fn main() {
    let (total_paper, total_ribbon) = iter_input_regex(r"(\d+)x(\d+)x(\d+)").fold(
        (0, 0), |(paper, ribbon), [l, w, h]: [u32; 3]| {
            (paper + 2 * (l*w + w*h + h*l) + l*w, ribbon + l*w*h + 2 * (l + w))
        });
    advtools::print("Paper", total_paper);
    advtools::print("Ribbon", total_ribbon);
}
