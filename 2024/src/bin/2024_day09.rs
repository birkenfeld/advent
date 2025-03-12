use advtools::input;
use advtools::prelude::Itertools;

fn main() {
    let mut disk = vec![];
    let input = input::string().chars().map(|ch| ch as u8 - b'0');
    for (id, (nfull, nfree)) in input.chain(Some(0)).tuples().enumerate() {
        disk.resize(disk.len() + nfull as usize, Some(id));
        disk.resize(disk.len() + nfree as usize, None);
    }

    // part 1 compaction: move everything into free blocks
    let mut disk1 = disk.clone();
    let mut i = 0;
    let mut j = disk1.len() - 1;
    while i != j {
        while disk1[i].is_none() {
            disk1.swap(i, j);
            j -= 1;
        }
        i += 1;
    }
    let checksum = disk1.iter().enumerate().map(|(i, &id)| i * id.unwrap_or(0)).sum::<usize>();
    advtools::verify("Checksum after compacting", checksum, 6395800119709_usize);

    // part 2 compaction: move files only contiguously
}
