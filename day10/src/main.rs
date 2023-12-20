use core::{Node, Coord};
use std::collections::{HashSet};
use itertools::{Itertools};
use crate::core::Pipe;

mod consts;
mod consts_two;
mod core;


fn main() {
    let parsed: Vec<Vec<Pipe>> = consts::BIG_INPUT.lines().enumerate()
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
        next = connected[*y][*x].connections.iter().find(|c| !pipe_loop.contains(*c));
        if let Some(l) = next {
            pipe_loop.insert(l);
        }
    }

    let mut num_inside = 0;
    for j in 0..connected.len() {
        let mut inside = false;
        for i in 0..connected[0].len() {
            if pipe_loop.contains(&connected[j][i].coords)  {
                if let '|' | 'L' | 'J'  = connected[j][i].pipe.char {
                    inside = !inside;
                }
            } else if inside {
                num_inside += 1
            }
        }
    }

    println!("num_inside {num_inside}");
}