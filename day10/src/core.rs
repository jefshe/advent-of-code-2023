use std::{cell::RefCell, collections::HashSet, rc::Rc};
use array2d::Array2D;
use Dir::{E,N,S,W};

pub type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W
}

#[derive(Debug, Clone)]
pub struct Pipe {
    pub coords: Coord,
    pub pipe_type: HashSet<Dir>,
}

impl Pipe {
    pub fn new<'a>(c: char, coords: Coord) -> Pipe {
        let pipe_type = match c {
            '|' => vec![N, S],
            '-' => vec![E, W],
            'L'=> vec![N,E],
            'J'=> vec![N,W],
            '7'=> vec![S,W],
            'F'=> vec![S,E],
            'S'=> vec![N,S,E,W],
            '.' => vec![],
            _ => panic!("Unrecognized char {c}")
        };
        Pipe {
            pipe_type: pipe_type.into_iter().collect(),
            coords,
        }
    }

    pub fn connect_up<'a>(&self, map: &'a Array2D<Pipe>) -> Node {
        let (x,y) = self.coords;
        let mut connections: HashSet<Coord> = HashSet::new();
        for dir in self.pipe_type.iter() {
            let pipe = match dir {
                N => check(map.get(x, y+1), S),
                S => check(map.get(x, y-1), N),
                E => check(map.get(x+1, y), W),
                W => check(map.get(x-1, y), E),
            };

            if let Some(p) = pipe {
                connections.insert(p);
            }
        }

        Node {
            coords: self.coords,
            connections
        }
    }
}

fn check(pipe: Option<&Pipe>, connecting_dir: Dir) -> Option<Coord> {
    match pipe {
        Some(p) if p.pipe_type.contains(&connecting_dir) => Some(p.coords),
        _ => None
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub coords: Coord,
    pub connections: HashSet<Coord>
}