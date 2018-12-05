use std::collections::HashSet;

pub fn part_one() -> i32 {
    parse_input().iter().fold(0, |acc, x| acc + x)
}

pub fn part_two() -> i32 {
    let frequency_change_list = parse_input();
    let mut frequency = 0;
    let mut frequency_history = HashSet::new();
    frequency_history.insert(frequency);

    for frequency_change in frequency_change_list.iter().cycle() {
        frequency += frequency_change;
        if !frequency_history.insert(frequency) {
            break
        }
    }

    frequency
}

fn parse_input() -> Vec<i32> {
    let input = include_str!("inputs/day1.txt");

    input.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}