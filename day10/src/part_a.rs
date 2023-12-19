use std::collections::VecDeque;

use itertools::repeat_n;

use crate::core::{Pipe, Node, Coord};


pub fn part_a(connected: Vec<Vec<Node>>, start: &Pipe) {
    let mut queue : VecDeque<Coord> = VecDeque::new();
    let mut distances: Vec<Vec<Option<u32>>> = repeat_n( vec![None; connected[0].len()], connected.len(),).collect();
    distances[start.coords.1][start.coords.0] = Some(0);
    queue.push_front(start.coords);

    while let Some((x,y)) = queue.pop_front() {
        let node = &connected[y][x];
        for (con_x, con_y)in &node.connections {
            if let None = distances[*con_y][*con_x] {
                distances[*con_y][*con_x] = Some(distances[y][x].unwrap() + 1);
                queue.push_back((*con_x, *con_y));
            }
        }
    }

    let max_dist: Option<&u32> = distances.iter().flat_map(|r| r.iter().flatten().max()).max(); 
    println!("max {:?}",max_dist) //6823 
}