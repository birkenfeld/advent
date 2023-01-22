use advtools::input;

fn main() {
    let mut error_score = 0u64;
    let mut complete_scores = vec![];
    'lines: for line in input::lines() {
        // Keep a stack of expected closing braces.
        let mut expected = vec![];
        for ch in line.chars() {
            match ch {
                '(' => expected.push(')'),
                '[' => expected.push(']'),
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                _   => if expected.pop() != Some(ch) {
                    // Line is corrupt, calculate the error score.
                    error_score += match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!()
                    };
                    continue 'lines;
                },
            }
        }
        // Line is incomplete, calculate the completer score.
        complete_scores.push(
            expected.into_iter().rev().fold(0u64, |score, ch| 5*score + match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!()
            })
        );
    }

    // Get the median completer score for part 2.
    complete_scores.sort_unstable();
    let complete_score = complete_scores[complete_scores.len() / 2];

    advtools::verify("Syntax error score", error_score, 369105);
    advtools::verify("Complete score", complete_score, 3999363569u64);
}
