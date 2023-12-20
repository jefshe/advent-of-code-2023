use std::collections::{VecDeque, HashSet};
use itertools::{Itertools, repeat_n};

mod consts;


fn main() {
    let mut dish: Vec<Vec<char>> = consts::BIG_INPUT.lines().map(|l|l.chars().collect()).collect();
    roll_up(&mut dish);
    println!("total {}", total_load(&dish));
}

pub fn roll_up(dish: &mut Vec<Vec<char>>) {

    for i in 0..dish[0].len() {
    let mut roll_pos = 0;
    for j in 0..dish.len() {
        if dish[j][i] == 'O' && j > roll_pos  {
            dish[j][i] = '.';
            dish[roll_pos][i] = 'O';
            roll_pos+=1
        } else if let '#' | 'O' = dish[j][i] {
            roll_pos = j + 1
        }
    }
    }
}

pub fn total_load(rolled_dish: &Vec<Vec<char>>)  -> usize {
    let max_load = rolled_dish.len();
    let mut total_load = 0;
    for j in 0..rolled_dish.len() {
    for i in 0..rolled_dish[0].len() {
        if rolled_dish[j][i] == 'O' {
            total_load += max_load - j;
        }
    }
    }
    total_load
}