use crate::bitonic::SortOrder;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::cmp::Ordering;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted<T: Ord>(xs: &[T], ord: SortOrder) -> bool {
    xs.windows(2).all(|pair| {
        if ord == SortOrder::Ascending {
            pair[0] <= pair[1]
        } else {
            pair[0] >= pair[1]
        }
    })
}
