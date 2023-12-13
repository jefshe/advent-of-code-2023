use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;


mod consts;
mod part_a;

fn main() {
    let sequences: Vec<Vec<usize>> = consts::INPUT.lines().map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect()).collect();
    println!("{sequences:?}")
}
