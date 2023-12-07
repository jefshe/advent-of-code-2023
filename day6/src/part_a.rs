
#![feature(let_chains)]
use std::{collections::HashMap, ops::Range};

use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;


const race_times: [u32; 4] = [44, 82, 69, 81];
const race_records: [u32; 4] = [202,1076,1138,1458];
fn main() {
    let solution = race_times
        .into_iter()
        .zip(race_records)
        .fold(1, |acc, (time, record) | acc * num_solutions(time, record));
    println!("{solution}");

}

pub fn num_solutions(race_time: u32, race_record: u32) -> u32 {
    let mut num_solutions = 0;
    for hold_time in 0..race_time {
        // speed * remaining_time
        if hold_time * (race_time - hold_time) > race_record {
            num_solutions += 1
        }
    }
    println!("race_time {race_time}, race_record: {race_record}: {num_solutions}");
    num_solutions
}
