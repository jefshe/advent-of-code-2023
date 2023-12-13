use std::collections::VecDeque;

use itertools::Itertools;


pub type Sequence = Vec<i32>;
pub fn part_a(pyramid: Vec<Sequence>) -> i32 {
    next_seq(&pyramid, 0)
}

fn next_seq(pyramid: &Vec<Sequence>, level: usize) -> i32 {
    if level == pyramid.len() - 2 {
        pyramid[level][0]
    } else {
        let s = pyramid[level].last().unwrap();
        s + next_seq(pyramid, level + 1)
    }
}