use std::{collections::btree_map::BTreeMap, cmp::Ordering};
use std::collections::HashMap;
use std::sync::OnceLock;
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

const CARD_RANK:[char; 13] = ['A','K','Q','T','9','8','7','6','5','4','3','2', 'J'];
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


#[derive(PartialOrd, PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<char>
}

impl Hand {
    pub fn new(hand: &str) -> Hand {
        Hand {
            cards: hand
            .chars().into_iter()
            .collect()
        }
    }

    pub fn get_hand_type(&self) -> HandType {
        let mut count = BTreeMap::new();
        for c in &self.cards {
            *count.entry(c).or_insert(0) += 1;
        }

        let n_wild_cards = match count.remove_entry(&'J') {
            None => 0,
            Some((_,cnt)) => cnt
        };

        let totals: Vec<u32> = count.into_values().sorted_by(|a,b| b.cmp(a)).collect();
        let one_card_best_total = totals.get(0).unwrap_or(&0) + n_wild_cards;
        let two_card_best_total = one_card_best_total + totals.get(1).unwrap_or(&0);

        match (one_card_best_total, two_card_best_total) {
            (5,_) => HandType::FiveOfAKind,
            (4,_) => HandType::FourOfAKind,
            (_, 5) => HandType::FullHouse,
            (3,_) => HandType::ThreeOfAKind,
            (_,4) => HandType::TwoPair,
            (2,_) => HandType::Pair,
            _ => HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.get_hand_type().cmp(&other.get_hand_type());
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }
        for (a, b) in self.cards.iter().zip(&other.cards) {
            let card_cmp = rank_lookup()[a].cmp(&rank_lookup()[b]);
            if card_cmp != Ordering::Equal {
                return card_cmp
            }
        }
        return Ordering::Equal;
    }
}