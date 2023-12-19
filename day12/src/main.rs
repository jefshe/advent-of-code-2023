use itertools::Itertools;
use part_a::find_combinations;

mod consts;
mod part_a;
mod core;


fn main() {
    let parsed: Vec<(Vec<char>, Vec<usize>)> = consts::BIG_INPUT.lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
        .map(|(status, constraint)| 
            (status.chars().collect_vec(), constraint.split(",").map(|c| c.parse().unwrap()).collect()))
        .collect();
    let sum: usize = parsed.iter()
        .flat_map(|(status, constraints)| find_combinations(status, constraints, false))
        .sum();
    println!("total {sum}")
    // for (status, constraint) in parsed {
    //     if let Some(combos) = find_combinations(&status, &constraint, false) {
    //         println!("{status:?} : {combos}")
    //     }
    // }
}