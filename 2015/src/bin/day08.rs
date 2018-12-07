use advtools::input::iter_input;

fn main() {
    let mut literal_len = 0;
    let mut memory_len = 0;
    let mut reescaped_len = 0;
    for line in iter_input::<String>() {
        let mut esc = false;
        let mut skip = 0;
        let mut extra_len = 0;
        for ch in line.chars().skip(1) {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if esc {
                match ch {
                    'x' => { skip = 2; }
                    '"' | '\\' => { extra_len += 1; }
                    _ => panic!("unknown escape {} in {}", ch, line)
                }
                esc = false;
                continue;
            } else if ch == '\\' {
                esc = true;
                extra_len += 1;
            } else if ch == '"' {
                break;
            }
            memory_len += 1;
        }
        literal_len += line.len();
        reescaped_len += line.len() + extra_len + 4;
    }
    // println!("Literal length: {}", literal_len);
    // println!("In-memory length: {}", memory_len);
    // println!("Re-escaped length: {}", reescaped_len);
    println!("Difference literal - memory: {}", literal_len - memory_len);
    println!("Difference reescaped - literal: {}", reescaped_len - literal_len);
}
