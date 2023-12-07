#![feature(let_chains)]
use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;
use std::{collections::HashMap};
use std::sync::OnceLock;

use crate::hand::{Hand, get_hand_type};
mod consts;
mod hand;

const CARD_RANK:[char; 13] = ['A','K','Q','J','T','9','8','7','6','5','4','3','2'];

fn rank_lookup() -> &'static HashMap<char, usize> {
    static LOOKUP: OnceLock<HashMap<char, usize>> = OnceLock::new();
    LOOKUP.get_or_init(|| {
        CARD_RANK
            .iter()
            .enumerate()
            .map(|(idx, card)| (card.to_owned(), 13 - idx))
            .collect()
    })
}

fn main() {
    let inputs :Vec<(&str, &str)> = consts::INPUT.iter().map(|line| line.split_once(" ").unwrap()).collect();
    let bid: Vec<u32> = inputs.iter().map(|(_, bid)| bid.parse().unwrap()).collect();
    let hands: Vec<Hand> = inputs
        .iter()
        .map(|(hand, _)| sort_hand(hand))
        .collect();
    for h in hands {
        println!("{:?}: {:?}", h, get_hand_type(&h));
    }
}

fn sort_hand(hand: &str) -> Hand {
    hand
        .chars().into_iter()
        .sorted_by_key(|a| &rank_lookup()[a])
        .collect()
}