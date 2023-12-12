#![feature(let_chains)]
use itertools::Itertools;
mod consts;
mod rectangle;

use rectangle::{Coord, Rectangle};

fn main() {
    let inputs :Vec<Vec<char>> = consts::BIG_INPUT.iter().map(|line| line.chars().collect_vec()).collect();
    let mut symbol_coords : Vec<Coord> = Vec::new();
    let mut numbers: Vec<(usize, Rectangle)> = Vec::new();
    let mut partial_number: String = String::new();
    for j in 0..inputs.len() {
        for i in 0..inputs[0].len() {
            let c = inputs[j][i];
            if is_digit(c)  {
                partial_number.push(c)
            } 
            
            if partial_number.len() > 0 && (i == inputs[0].len() - 1 || !is_digit(c))  {
                let end_coord = (if is_digit(c) { i } else { i - 1 }, j);
                numbers.push(parse_num(&partial_number, end_coord));
                partial_number.clear()
            }

            if is_symbol(c) {
                symbol_coords.push((i ,j));
            }
        }
    }
    let parts: Vec<usize> = numbers
        .iter()
        .filter(|(n, area)| symbol_coords.iter().any(|sym| {
            if area.is_inside(sym) {
                println!("{sym:?} is inside {n} [{area:?}]")
            }
            area.is_inside(sym)

            
        }))
        .map(|(n, _)|*n)
        .collect();
    let parts_sum: usize = parts.iter().sum();
    println!("{numbers:?}");
    println!("SUM: {parts_sum}");
}

fn parse_num(num_str: &str, end_coord: Coord) -> (usize, Rectangle) {
    let (i,j) = end_coord;
    let value = num_str.parse().unwrap();
    let area = Rectangle::surround(((i + 1 - num_str.len()), j), num_str.len());
    if value == 755 {
        println!("{area:?}, {i}, {j}");
    }
    (value, area)
}

fn is_symbol(c: char) -> bool {
    c != '.' && !is_digit(c)
}

fn is_digit(c: char) -> bool {
    c >= '0'&& c <= '9'
}

