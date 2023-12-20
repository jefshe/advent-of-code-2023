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
    pub pipe_type: Vec<Dir>,
    pub char: char
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
            pipe_type,
            coords,
            char: c
        }
    }

    pub fn connect_up<'a>(&self, map: &'a Vec<Vec<Pipe>>) -> Node {
        let connections: HashSet<Coord> = self.pipe_type.iter().flat_map(|dir| self.check_pipe(dir, map)).collect();
        Node {
            coords: self.coords,
            connections,
            pipe: map[self.coords.1][self.coords.0].clone()
        }
    }

    fn check_pipe<'a, 'b>(self: &Self, dir: &'a Dir, map: &'b Vec<Vec<Pipe>>) -> Option<Coord> {
        let (x,y) = self.coords;
        // TODO: why is S connected
        match dir {
            N if y > 0 => check(map.get(y-1)?.get(x), S),
            S => check(map.get(y+1)?.get(x), N),
            E => check(map.get( y)?.get(x+1), W),
            W if x > 0 => check(map.get(y)?.get(x-1), E),
            _ => None
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
    pub connections: HashSet<Coord>,
    pub pipe: Pipe
}