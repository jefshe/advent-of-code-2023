use itertools::Itertools;

use crate::consts;

pub fn part_a() {
    let parsed: Vec<(Vec<char>, Vec<usize>)> = consts::BIG_INPUT.lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
        .map(|(status, constraint)|
            (status.chars().collect_vec(), constraint.split(",").map(|c| c.parse().unwrap()).collect()))
        .collect();
    let sum: usize = parsed.iter()
        .flat_map(|(status, constraints)| find_combinations(status, constraints, false))
        .sum();
    println!("total {sum}")
}

fn find_combinations(springs: &[char], constraints: &[usize], in_streak: bool) -> Option<usize> {
    match (springs, constraints) {
        ([], [0]) => { Some(1) },
        ([], []) => { Some(1) },
        ([], _) => {
            None
        },
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