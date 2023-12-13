use std::{collections::{HashMap, HashSet}, cell::RefCell, rc::Rc};
use itertools::Itertools;
use array2d::Array2D;

use crate::core::{Pipe};

mod consts;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<Vec<Rc<Pipe>>> = consts::INPUT.lines().enumerate()
        .map(|(y, l)| 
            l.chars().enumerate()
            .map(|(x, c)|Rc::new(Pipe::new(c, (x, y)))).collect())
        .collect();
    let pipe_map= Array2D::from_rows(&parsed).unwrap();


    
    for x in 0..pipe_map.row_len() {
    for y in 0..pipe_map.column_len() {

    }
    }
}