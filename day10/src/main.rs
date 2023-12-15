use core::{Node, Coord};
use std::collections::{BinaryHeap, VecDeque};
use itertools::Itertools;
use array2d::Array2D;
use crate::core::Pipe;

mod consts;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<Vec<Pipe>> = consts::MEDIUM_INPUT.lines().enumerate()
        .map(|(y, l)| 
            l.chars().enumerate()
            .map(|(x, c)|Pipe::new(c, (x, y))).collect())
        .collect();

    let pipe_map= Array2D::from_rows(&parsed).unwrap();
    let connected: Vec<Vec<Node>> = pipe_map.columns_iter().map(|r| r.map(|p| p.connect_up(&pipe_map)).collect()).collect_vec();
    let connected_map = Array2D::from_rows(&connected).unwrap();
    let mut distances: Array2D<Option<u32>> = Array2D::filled_with(None, connected_map.num_rows(), connected_map.num_columns());
    let mut queue : VecDeque<(Coord, u32)> = VecDeque::new();
    let start = pipe_map.columns_iter().enumerate().find_map(|(y, row)| 
        match row.enumerate().find_map(|(x, p)| if p.pipe_type.len() == 4 { Some(x) } else { None }) {
            None => None,
            Some(x) => Some((x, y))
        }
    ).unwrap();


    queue.push_front((start, 0));
    while let Some((coord, distance)) = queue.pop_front() {
        let node = &connected_map[coord];
        distances[coord] = Some(distance);
        for connected in &node.connections {
            if let None = distances[*connected] {
                distances[*connected] = Some(distance + 1);
                queue.push_back((connected.to_owned(), distance + 1));
            }
        }
    }

    for i in 0..connected_map.row_len() {
        for j in 0..connected_map.column_len() {
            print!("{:?}{} " , connected_map[(i,j)].pipe.char, if connected_map[(i,j)].connections.len() > 0 { "!"} else {" "})
        }
        println!()
    }



    for i in 0..distances.row_len() {
        for j in 0..distances.column_len() {
            print!("{} " , if let Some(num) = distances[(i,j)] { num.to_string() } else { String::from(".") })
        }
        println!()
    }
}