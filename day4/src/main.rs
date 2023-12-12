mod consts;
use std::collections::HashSet;
type Game = (HashSet<usize>, Vec<usize>);
fn main() {
    let inputs: Vec<(HashSet<usize>, Vec<usize>)> = consts::BIG_INPUT
        .into_iter()
        .map(|s| {
            let l_r: Vec<&str> = s.split("|").collect();
            let [a,b ]: [&str] = l_r[0..2] else {panic!()};
            let winning: HashSet<usize> = unsafe_parse(a, 2);
            let numbers: Vec<usize> = unsafe_parse(b, 0);
            (winning, numbers)
        })
        .collect();
    part_b(inputs)
}

fn part_b(games: Vec<Game>) {
    let mut card_count: Vec<usize> = std::iter::repeat(1).take(games.len()).collect();
    for (base_idx, g )in games.iter().enumerate() {
        for extra_copies in 0..get_matches(g) {
            let game_idx = base_idx + extra_copies + 1;
            card_count[game_idx] += card_count[base_idx];
        }
    }
    let total: usize = card_count.iter().sum();
    println!("total: {total}", )

}

fn part_a(games: Vec<Game>) {
    let mut points = 0;
    for g in games {
        let matches = get_matches(&g);
        if matches > 0 {
            points += 1 << (matches-1)
        }
    }
    println!("points: {points}")
}

fn get_matches(game: &Game) -> usize {
    let (win_set, nums) = game;
    nums.iter().filter(|n| win_set.contains(n)).count()
}

fn unsafe_parse<B: FromIterator<usize>>(line:  &str, skip: usize) -> B {
    line.split_whitespace().skip(skip).map(|str| str.parse().expect(&format!("could not parse: {str}"))).collect()
}
