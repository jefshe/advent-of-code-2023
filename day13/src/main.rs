#![feature(let_chains)]
use std::{collections::HashMap, iter};
use itertools::Itertools;

mod consts;
mod part_a;
use consts::{INPUT, BIG_INPUT};

type Coord = (usize, usize);
fn main() {
    let patterns: Vec<Vec<Vec<char>>> = BIG_INPUT.lines().map(|l| l.chars().collect()).peekable().batching(|it| 
        match it.peek() {
            Some(_) => Some(it.take_while(|c: &Vec<char>| c.len() > 0).collect()),
            None => None
        }
    )
    .collect();

    let mut total = 0;
    for p in patterns {
        let col_reflections = find_reflections(&p);
        let row_reflections = find_reflections(&transpose(p));
        total += col_reflections + row_reflections * 100;
    }
    println!("total: {total}", )
}

fn find_reflections(pat: &Vec<Vec<char>>) -> usize {
    let mut total_reflection = 0;
    for i in 0..pat[0].len() {
    let mut reflection = true;
    for j in 0..pat.len() {
        reflection &= check_reflect(&pat[j], i)
    }
    if reflection {
       total_reflection += i + 1
    }
    }
    total_reflection
}

fn check_reflect(pat: &Vec<char>, pivot: usize) -> bool {
    let (mut i, mut j) = (pivot as i32, (pivot+1) as i32);
    let mut reflection = false;
    while i >= 0 && j >= 0 && let Some(c1) = pat.get(i as usize) && let Some(c2) = pat.get(j as usize) {
        if c1 != c2 {
            return false
        }
        reflection = true;
        i -= 1;
        j += 1;
    }
    reflection
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}