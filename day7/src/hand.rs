use std::collections::btree_map::BTreeMap;

use itertools::Itertools;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
pub type Hand = Vec<char>;

pub fn get_hand_type(hand: &Hand) -> HandType {
    let mut count = BTreeMap::new();
    for h in hand {
        *count.entry(h).or_insert(0) += 1;
    }
    let totals: Vec<u32> = count.into_values().sorted_by(|a,b| b.cmp(a)).collect();

    match totals.as_slice() {
        [5, ..] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2, ..] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::Pair,
        _ => HandType::HighCard
    }
}
