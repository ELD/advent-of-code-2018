use std::fs::File;
use std::io::Read;

pub fn part_one() -> usize {
    let resulting_polymer = react_polymer(get_polymer_chain());

    resulting_polymer.len()
}

pub fn part_two() -> usize {
    let polymer_chain = get_polymer_chain();

    let mut reacted_polymers = Vec::new();
    for c in b'A'..=b'Z' {
        let c1 = c + 32;
        reacted_polymers.push(react_polymer(polymer_chain.replace(c as char, "").replace(c1 as char, "")).len());
    }

    *reacted_polymers.iter().min().unwrap()
}

fn get_polymer_chain() -> String {
    let mut file = File::open("src/inputs/day5.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    buffer
}

fn react_polymer(polymer: String) -> String {
    let mut reaction_happened = true;
    let mut reacted_polymer = polymer.as_bytes().to_vec();

    while reaction_happened {
        let mut i = 1;
        reaction_happened = false;
        while i < reacted_polymer.len() {
            if reacts(reacted_polymer[i], reacted_polymer[i-1]) {
                reacted_polymer.remove(i);
                reacted_polymer.remove(i-1);
                reaction_happened = true;
                continue;
            }

            i += 1;
        }
    }

    String::from_utf8_lossy(&reacted_polymer).into()
}

fn reacts(a: u8, b: u8) -> bool {
    if a > b {
        a - b == 32
    } else {
        b - a == 32
    }
}
