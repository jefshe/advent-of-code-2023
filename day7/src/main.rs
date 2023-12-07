#![feature(let_chains)]
use itertools::Itertools;

use crate::hand::Hand;
mod consts;
mod hand;

fn main() {
    let inputs :Vec<(&str, &str)> = consts::BIG_INPUT.iter().map(|line| line.split_once(" ").unwrap()).collect();
    let hands: Vec<(Hand, usize)> = inputs
        .iter()
        .map(|(hand, bid)| (Hand::new(hand), bid.parse().unwrap()))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect();
    let solution: usize = hands.into_iter().enumerate().map(|(i, (_, bid))| bid * (i+1)).sum();
    println!("solution: {solution}")

}

