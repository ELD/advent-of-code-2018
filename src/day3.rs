use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use regex::Regex;

type Grid = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn iter_claim(&self) -> IterClaim {
        IterClaim {
            claim: self,
            px: self.x,
            py: self.y,
        }
    }
}

struct IterClaim<'a> {
    claim: &'a Claim,
    px: u32,
    py: u32,
}

impl<'a> Iterator for IterClaim<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.py >= self.claim.y + self.claim.height {
            self.py = self.claim.y;
            self.px += 1
        }

        if self.px >= self.claim.x + self.claim.width {
            return None;
        }

        let (px, py) = (self.px, self.py);
        self.py += 1;
        Some((px, py))
    }
}

pub fn part_one() -> usize {
    let claims = parse_input();

    let grid = setup_grid(&claims);

    grid.values().filter(|&&count| count > 1).count()
}

pub fn part_two() -> u32 {
    let claims = parse_input();

    let grid = setup_grid(&claims);

    claims.iter().filter_map(|claim| {
        if claim.iter_claim().all(|point| grid[&point] == 1) { Some(claim.id) } else { None }
    }).nth(0).unwrap()
}

fn parse_input() -> Vec<Claim> {
    lazy_static!(
static ref CLAIM_REGEX: Regex = Regex::new(
r"(?x)\#(?P<id>\d+)\s+@\s+(?P<x>\d+),(?P<y>\d+):\s+(?P<width>\d+)x(?P<height>\d+)"
).unwrap();
);

    let mut buffer = String::new();
    let mut file = File::open("src/inputs/day3.txt").unwrap();
    file.read_to_string(&mut buffer).unwrap();
    buffer.split('\n').filter_map(|line| {
        if let Some(caps) = CLAIM_REGEX.captures(line) {
            return Some(Claim {
                id: caps["id"].parse().unwrap(),
                x: caps["x"].parse().unwrap(),
                y: caps["y"].parse().unwrap(),
                width: caps["width"].parse().unwrap(),
                height: caps["height"].parse().unwrap(),
            });
        }

        None
    }).collect()
}

fn setup_grid(claims: &Vec<Claim>) -> Grid {
    let mut grid = Grid::new();

    for claim in claims {
        for (x, y) in claim.iter_claim() {
            *grid.entry((x, y)).or_default() += 1;
        }
    }

    grid
}
