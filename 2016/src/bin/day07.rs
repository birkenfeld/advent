use advtools::prelude::Itertools;
use advtools::input;

fn has_abba(s: &&str) -> bool {
    s.chars().tuple_windows().any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn all_abas(s: &&str) -> Vec<(char, char, char)> {
    s.chars().tuple_windows().filter(|(a, b, c)| a == c && a != b).collect()
}

fn has_bab(s: &str, aba: (char, char, char)) -> bool {
    s.chars().tuple_windows().any(|(a, b, c)| a == aba.1 && a == c && b == aba.0)
}

fn main() {
    let mut supports_tls = 0;
    let mut supports_ssl = 0;
    for line in input::lines() {
        let (sup, hyp): (Vec<_>, Vec<_>) = line.split(|c| c == '[' || c == ']')
                                               .enumerate()
                                               .partition(|(i, _)| i % 2 == 0);
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
    advtools::verify("Addresses with TLS support", supports_tls, 118);
    advtools::verify("Addresses with SSL support", supports_ssl, 260);
}
