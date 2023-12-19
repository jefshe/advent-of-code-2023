use core::{Node, Coord};
use std::collections::{VecDeque, HashSet};
use itertools::{Itertools, repeat_n};
use crate::core::Pipe;

mod consts;
mod consts_two;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<Vec<Pipe>> = consts_two::INPUT.lines().enumerate()
        .map(|(y, l)| 
            l.chars().enumerate()
            .map(|(x, c)|Pipe::new(c, (x, y))).collect())
        .collect();
    let connected: Vec<Vec<Node>> = parsed.iter().map(|r| r.iter().map(|p| p.connect_up(&parsed)).collect()).collect_vec();
    let start = connected.iter().find_map(|row| row.iter().find(|p| p.pipe.char == 'S')).unwrap();

    let mut next: Option<&Coord> = Some(&start.coords);
    let mut pipe_loop: HashSet<&Coord> = vec![next.unwrap()].into_iter().collect();
    while let Some(coords) = next {
        let (x,y) = coords;
        println!("next {:?}", next);
        println!("connections {:?}", connected[*y][*x].connections);
        next = connected[*y][*x].connections.iter().find(|c| !pipe_loop.contains(*c));
        if let Some(l) = next {
            pipe_loop.insert(l);
        }
    }

    println!("{pipe_loop:?}");
    for j in 0..connected.len() {
        for i in 0..connected[0].len() {
            print!("{}{}", connected[j][i].pipe.char, if pipe_loop.contains(&connected[j][i].coords) { '!' } else {' '})
        }
        println!(" ");
    }
}