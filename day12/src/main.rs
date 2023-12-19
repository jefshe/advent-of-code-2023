use std::collections::HashMap;

use cached::proc_macro::cached;
use cached::UnboundCache;
use itertools::Itertools;

mod consts;
mod part_a;


fn main() {
    let parsed: Vec<(Vec<char>, Vec<usize>)> = consts::BIG_INPUT.lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
        .map(|(status, constraint)| 
            (status.chars().collect_vec(), constraint.split(",").map(|c| c.parse::<usize>().unwrap()).collect()))
        .map(|(springs, constraints)| parse_part_b(springs, constraints))
        .collect();
    let sum: usize = parsed.iter()
        .flat_map(|(status, constraints)| solve_part_b(status, constraints))
        //.for_each(|combs| println!("combs: {combs:?}"));
        .sum();
    println!("total {sum}")
}

pub fn parse_part_b(springs: Vec<char>, constraints: Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let repeated_springs_len = (springs.len() + 1) * 5 - 1; // Don't want the last '?'
    let repeated_constraints_len = constraints.len() * 5;
    (
        springs.into_iter().chain(['?']).cycle().take(repeated_springs_len).collect(),
        constraints.into_iter().cycle().take(repeated_constraints_len).collect()
    )
}

pub fn solve_part_b(springs: &[char], constraints: &[usize]) -> Option<usize> {
    find_combinations(springs, constraints, false)
}


#[cached(
    type = "UnboundCache<String, Option<usize>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}{:?}{}", springs, constraints, in_streak) }"#
)]
fn find_combinations(springs: &[char], constraints: &[usize], in_streak: bool) -> Option<usize> {
    match (springs, constraints) {
        ([], [0]) => Some(1),
        ([], []) => Some(1),
        ([], _) => None,
        ([char, statuses @ ..], []) => {
            if *char == '#' {
                None
            } else {
                find_combinations(statuses, &[], false)
            }
        },
        (['#', statuses @ ..], [limit, remaining @ ..]) => {
            if *limit == 0 {
                None
            } else {
                let mut reduced_constraint = vec![limit-1;remaining.len() +1];
                reduced_constraint[1..].copy_from_slice(remaining);
                find_combinations(statuses, &reduced_constraint, true)
            }
        },
        (['.', statuses @ ..], [limit, remaining @ ..]) => {
            if *limit == 0 {
                find_combinations(statuses, remaining, false)
            } else if in_streak {
                None
            } else {
                find_combinations(statuses, constraints, false)
            }
        },
        (['?', statuses @ ..], constraint) => {
            let mut damaged = vec!['#'; statuses.len() +1];
            let mut operational = vec!['.'; statuses.len() +1];
            damaged[1..].copy_from_slice(statuses);
            operational[1..].copy_from_slice(statuses);
            let operational_branch = find_combinations(&operational, constraint, in_streak);
            let damaged_branch = find_combinations(&damaged, constraint, in_streak);
            let sum = damaged_branch.into_iter().chain(operational_branch).sum();
            Some(sum)
        },
        _ => None
    }
}