#![feature(let_chains)]
mod part_a;
use itertools::Itertools;
use std::cmp::{max,min};
use std::collections::VecDeque;

use std::{collections::HashMap, ops::Range};


const RACE_TIME: f64 = 44826981.0;
const RACE_RECORD: f64 = 202107611381458.0;
// const RACE_TIME: f64 = 71530.0;
// const RACE_RECORD: f64 = 940200.0;

fn main() {

    // -x**2 + RACE_TIME*x - RACE_RECORD = 0
    let (a, b)= roots_of_poly(-1.0, RACE_TIME, -RACE_RECORD);
    println!("roots: {a}, {b}");
    let upper = a.max(b).floor();
    let lower = a.min(b).ceil();
    println!("solutions: {} = {upper} - {lower} + 1", upper - lower + 1.0);
}

fn roots_of_poly(a: f64, b:f64, c:f64) -> (f64, f64) {
    let discriminant = b.powi(2) - (4.0 * a * c);
    let root1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
    let root2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
    (root1, root2)
}
