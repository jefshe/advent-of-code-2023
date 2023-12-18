use crate::core::Status;
use crate::core::Status::{Operational, Damaged, Unknown};
use std::borrow::Cow;


pub fn find_combinations<'a, T: Into<Cow<'a, [u8]>>>(springs: &[Status], constraint: &[usize]) -> Option<usize> {
    println!("{springs:?} {constraint:?}");
    match (springs, constraint) {
        ([], []) => Some(1),
        ([], _) => None,
        ([Operational, statuses @ ..], [limit, constraint @ ..]) => {
            if *limit == 0 {
                None
            } else {
                let mut reduced_constraint = vec![limit-1;constraint.len() +1];
                reduced_constraint[1..].copy_from_slice(constraint);
                println!("reduced: {reduced_constraint:?}");
                println!("old: {constraint:?}");
                find_combinations(statuses, &reduced_constraint)
            }
        },
        ([Damaged, statuses @ ..], constraint) => {
            find_combinations(statuses, constraint)
        },
        ([Unknown, statuses]) => {
        },
        _ => None
    }
}