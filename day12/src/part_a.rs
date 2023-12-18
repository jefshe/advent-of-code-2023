use crate::core::Status;
use crate::core::Status::{Operational, Damaged, Unknown};


pub fn find_combinations(springs: &[Status], constraints: &[usize], in_streak: bool) -> Option<usize> {
    match (springs, constraints) {
        ([], []) => Some(1),
        ([], _) => None,
        ([Operational, statuses @ ..], [limit, remaining @ ..]) => {
            if *limit == 0 {
                None
            } else {
                let mut reduced_constraint = vec![limit-1;remaining.len() +1];
                reduced_constraint[1..].copy_from_slice(remaining);
                find_combinations(statuses, &reduced_constraint, true)
            }
        },
        ([Damaged, statuses @ ..], [limit, remaining @ ..]) => {
            // TODO eliminate streaks here
            if *limit == 0 {
                find_combinations(statuses, remaining, false)
            } else if in_streak {
                None
            } else {
                find_combinations(statuses, constraints, false)
            }
        },
        ([Unknown, statuses @ ..], constraint) => {
            let mut damaged = vec![Status::Damaged; statuses.len() +1];
            let mut operational = vec![Status::Operational; statuses.len() +1];
            damaged[1..].copy_from_slice(statuses);
            operational[1..].copy_from_slice(statuses);
            let operational_branch = find_combinations(&operational, constraint, false);
            let damaged_branch = find_combinations(&damaged, constraint, false);
            let sum = damaged_branch.into_iter().chain(operational_branch).sum();
            Some(sum)
        },
        _ => None
    }
}