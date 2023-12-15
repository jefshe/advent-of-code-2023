use core::Node;
use std::{collections::{HashMap, HashSet}, cell::RefCell, rc::Rc};
use itertools::Itertools;
use array2d::Array2D;

use crate::core::{Pipe};

mod consts;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<Vec<Pipe>> = consts::INPUT.lines().enumerate()
        .map(|(y, l)| 
            l.chars().enumerate()
            .map(|(x, c)|Pipe::new(c, (x, y))).collect())
        .collect();
    let pipe_map= Array2D::from_columns(&parsed).unwrap();
    let connected: Vec<Vec<Node>> = pipe_map.rows_iter().map(|r| r.map(|p| p.connect_up(&pipe_map)).collect()).collect_vec();
    let connected_map = Array2D::from_rows(&connected).unwrap();

    for x in 0..connected_map.row_len() {
    for y in 0..connected_map.column_len() {
    }
    }
}