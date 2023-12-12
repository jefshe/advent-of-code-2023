#![feature(let_chains)]
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

use crate::lr_node::LRNode;

mod consts;
mod lr_node;

fn main() {
    let split_re = Regex::new(r"[^A-Z0-9]+").expect("Invalid regex");

    let lookup : HashMap<&str, LRNode> = consts::BIG_INPUT.into_iter()
        .map(|line| {
            split_re.split(line).collect()
        })
        .map(|nodes: Vec<&str>| (nodes[0], LRNode::new(nodes[1], nodes[2])))
        .collect();

    let mut path = consts::BIG_PATH.chars().cycle();
    let mut nodes: Vec<_> = lookup.keys()
        .filter(|node| node.ends_with("A"))
        .map(|s| *s)
        .collect();
    let mut steps: usize = 0;

    let mut visited: HashMap<usize, usize> = HashMap::new();

    while !nodes.iter().all(|n| n.ends_with("Z")) {
        let step = path.next().unwrap();
        let prev = nodes.clone();
        nodes = nodes.iter().map(|n| lookup[*n].next(step)).collect();
        steps += 1;
        for (idx, (a,b)) in prev.iter().zip(&nodes).enumerate() {
            if b.ends_with("Z") {//&& !visited.insert(format!("{a} -> {b}")) {
                if !visited.contains_key(&idx) {
                    visited.insert(idx, steps);
                }
            }
        }

        if visited.len() >= 6 {
            break;
        }
    }

    let mut lcm_solution = 1;
    for v in visited.values() {
        lcm_solution = lcm(lcm_solution, *v);
    }

    println!("solution: {lcm_solution}")


}


fn part_a() {
    const START: &str = "AAA";
    const END: &str = "ZZZ";
    let split_re = Regex::new(r"[^A-Z]+").expect("Invalid regex");

    let lookup : HashMap<&str, LRNode> = consts::BIG_INPUT.into_iter()
        .map(|line| {
            split_re.split(line).collect()
        })
        .map(|nodes: Vec<&str>| (nodes[0], LRNode::new(nodes[1], nodes[2])))
        .collect();

    let mut path = consts::BIG_PATH.chars().cycle();
    let mut node = START;
    let mut steps = 0;
    while node != END {
        node = lookup[node].next(path.next().unwrap());
        steps += 1;
    }
    println!("num steps: {steps}")
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}