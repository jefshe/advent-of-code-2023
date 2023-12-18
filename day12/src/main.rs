use core::Status;
use std::{collections::{HashMap, HashSet}, cell::RefCell, rc::Rc};
use itertools::Itertools;
use part_a::find_combinations;

mod consts;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<(Vec<Status>, Vec<usize>)> = consts::INPUT.lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
        .map(|(status, constraint)| 
            (status.chars().map(Status::new).collect_vec(), constraint.split(",").map(|c| c.parse().unwrap()).collect()))
        .collect();
    for (status, constraint) in parsed {
        find_combinations(status, constraint)
    }
}