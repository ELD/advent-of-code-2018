use std::collections::HashMap;

pub fn part_one() -> i32 {
    let barcodes = parse_input();

    let mut two_letter_count = 0;
    let mut three_letter_count = 0;
    for barcode in barcodes {
        let mut letter_map = HashMap::new();
        barcode.chars().for_each(|c| { letter_map.entry(c).and_modify(|v| *v += 1).or_insert(1); });

        if letter_map
            .iter()
            .filter(|(_, v)| **v == 2)
            .count() > 0 {
            two_letter_count += 1;
        }

        if letter_map
            .iter()
            .filter(|(_, v)| **v == 3)
            .count() > 0 {
            three_letter_count += 1;
        }
    }

    two_letter_count * three_letter_count
}

pub fn part_two() -> String {
    let barcodes = parse_input();
    for i in 0..barcodes.len() {
        for j in i + 1..barcodes.len() {
            if let Some(common_letters) = common_letters(&barcodes[i], &barcodes[j]) {
                return common_letters;
            }
        }
    }

    "NOTHING FOUND".to_string()
}

fn parse_input() -> Vec<&'static str> {
    include_str!("inputs/day2.txt").split("\n").collect::<Vec<&str>>()
}

fn common_letters(lhs: &str, rhs: &str) -> Option<String> {
    if lhs.len() != rhs.len() {
        return None;
    }

    let mut one_common = false;
    for (l, r) in lhs.chars().zip(rhs.chars()) {
        if l != r {
            if one_common {
                return None;
            }

            one_common = true;
        }
    }

    Some(
        lhs.chars().zip(rhs.chars())
            .filter(|&(l, r)| l == r)
            .map(|(l, _)| l)
            .collect()
    )
}