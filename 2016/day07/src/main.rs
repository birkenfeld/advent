extern crate advtools;
extern crate itertools;

use itertools::Itertools;

fn has_abba(s: &&str) -> bool {
    s.chars().tuple_windows().any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn all_abas(s: &&str) -> Vec<(char, char, char)> {
    s.chars().tuple_windows().filter(|&(a, b, c)| a == c && a != b).collect()
}

fn has_bab(s: &str, aba: (char, char, char)) -> bool {
    s.chars().tuple_windows().any(|(a, b, c)| a == aba.1 && a == c && b == aba.0)
}

fn main() {
    let mut supports_tls = 0;
    let mut supports_ssl = 0;
    for line in advtools::iter_input::<String>() {
        let (sup, hyp): (Vec<_>, Vec<_>) = line.split(|c| c == '[' || c == ']')
                                               .enumerate()
                                               .partition(|&(i, _)| i % 2 == 0);
        let sup = sup.into_iter().map(|(_, s)| s).collect_vec();
        let hyp = hyp.into_iter().map(|(_, s)| s).collect_vec();
        if sup.iter().any(has_abba) && !hyp.iter().any(has_abba) {
            supports_tls += 1;
        }
        if sup.iter().flat_map(all_abas).cartesian_product(&hyp)
                                        .any(|(aba, s)| has_bab(s, aba)) {
            supports_ssl += 1;
        }
    }
    println!("Addresses with TLS support: {}", supports_tls);
    println!("Addresses with SSL support: {}", supports_ssl);
}
