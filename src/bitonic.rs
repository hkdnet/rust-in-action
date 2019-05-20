use crate::bitonic::SortOrder::{Ascending, Descending};

pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn sort<T: Ord>(xs: &mut [T], ord: SortOrder) {
    match ord {
        Ascending => do_sort(xs, true),
        Descending => do_sort(xs, false),
    }
}

fn do_sort<T: Ord>(xs: &mut [T], up: bool) {
    if xs.len() > 1 {
        let mid_point = xs.len() / 2;
        sort(&mut xs[..mid_point], Ascending);
        sort(&mut xs[mid_point..], Descending);
        sub_sort(xs, up)
    }
}

fn sub_sort<T: Ord>(xs: &mut [T], up: bool) {
    if xs.len() > 1 {
        compare_and_swap(xs, up);
        let mid_point = xs.len() / 2;
        sub_sort(&mut xs[..mid_point], up);
        sub_sort(&mut xs[mid_point..], up)
    }
}

fn compare_and_swap<T: Ord>(xs: &mut [T], up: bool) {
    let mid_point = xs.len() / 2;
    for i in 0..mid_point {
        if (xs[i] > xs[mid_point + i]) == up {
            xs.swap(i, mid_point + i)
        }
    }
}

#[cfg(test)]
mod test {
    use super::sort;
    use crate::bitonic::SortOrder::*;

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, Ascending);
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, Descending);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        sort(&mut x, Ascending);
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        sort(&mut x, Descending);
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }
}
