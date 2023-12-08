#![feature(let_chains)]
use std::collections::HashMap;
use regex::Regex;

use crate::lr_node::LRNode;

mod consts;
mod lr_node;

const START: &str = "AAA";
const END: &str = "ZZZ";

fn main() {
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



