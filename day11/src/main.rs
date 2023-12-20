use std::{collections::HashMap, iter};
use itertools::Itertools;

mod consts;
mod part_a;
use consts::BIG_INPUT;


type Coord = (usize, usize);
// PART A
// const EMPTY_DISTANCE: usize = 1;
const EMPTY_DISTANCE: usize = 999_999;

fn main() {
    let map: Vec<Vec<char>> = BIG_INPUT.lines().map(|l| l.chars().collect_vec()).collect();
    let mut extra_distance = 0;
    let mut galaxies: HashMap<Coord, Coord> = HashMap::new();
    for j in 0..map.len() {
    let mut empty_count = 0;
    for i in 0..map[0].len() {
        if map[j][i] == '#' {
            galaxies.insert((i,j), (i,j + extra_distance));
        } else {
            empty_count += 1
        }
    }
    if empty_count == map[0].len() {
        extra_distance += EMPTY_DISTANCE
    }
    }

    extra_distance = 0;
    for i in 0..map[0].len() {
    let mut empty_count = 0;
    for j in 0..map.len() {
        if map[j][i] == '#' {
            galaxies.insert((i,j), (galaxies[&(i,j)].0 + extra_distance, galaxies[&(i,j)].1));
        } else {
            empty_count += 1
        }
    }
    if empty_count == map.len() {
        extra_distance += EMPTY_DISTANCE
    }
    }



    let mut total = 0;
    for pair in galaxies.values().enumerate().combinations(2) {
        if let [(g1, (x1, y1)), (g2, (x2, y2))] = pair[..2] {
            let distance = y2.abs_diff(*y1) + x2.abs_diff(*x1);
            // println!("galaxy {} and {}: {distance}", g1+1, g2+1);
            total += distance
        } else {
            panic!()
        }
    }
    println!("total {total}")
}