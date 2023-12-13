use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use part_a::{part_a, Sequence};

mod consts;
mod part_a;

fn main() {
    let sequences: Vec<Sequence> = consts::INPUT.lines().map(|l| l.split_whitespace().map(|c| c.parse().expect(c)).collect()).collect();
    // part_a
    // let total: i32 = sequences.into_iter().map(build_pyramid).map(part_a).sum();
    let total: i32 = sequences.into_iter().map(build_pyramid).map(|p| prev_seq(p, 0)).sum();
    println!("{total}")
}

fn build_pyramid(seq: Sequence) -> Vec<Sequence> {
    let mut pyramid: Vec<Sequence> = vec![seq];
    let mut curr = pyramid.last().unwrap();
    while pyramid.last().unwrap().iter().any(|v| *v != 0) {
        let next_seq = curr.iter().tuple_windows().map(|(a,b)| b -a).collect();
        pyramid.push(next_seq);
        curr = pyramid.last().unwrap();
    }
    pyramid
}

fn prev_seq(pyramid: Vec<Sequence>, level: usize) -> i32{
    if level == pyramid.len() - 2 {
        pyramid[level][0]
    } else {
        pyramid[level][0] - prev_seq(pyramid, level + 1)
    }
}