use crate::bitonic::SortOrder::{Ascending, Descending};
use std::cmp::Ordering;

pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn sort<T: Ord>(xs: &mut [T], ord: SortOrder) -> Result<(), String> {
    match ord {
        Ascending => sort_by(xs, &|a, b| a.cmp(b)),
        Descending => sort_by(xs, &|a, b| b.cmp(a)),
    }
}

pub fn sort_by<T, F>(xs: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if xs.len().is_power_of_two() {
        do_sort(xs, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of xs is not a power of two. (xs.len(): {})",
            xs.len()
        ))
    }
}

fn do_sort<T, F>(xs: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if xs.len() > 1 {
        let mid_point = xs.len() / 2;
        do_sort(&mut xs[..mid_point], true, comparator);
        do_sort(&mut xs[mid_point..], false, comparator);
        sub_sort(xs, up, comparator)
    }
}

fn sub_sort<T, F>(xs: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if xs.len() > 1 {
        compare_and_swap(xs, up, comparator);
        let mid_point = xs.len() / 2;
        sub_sort(&mut xs[..mid_point], up, comparator);
        sub_sort(&mut xs[mid_point..], up, comparator);
    }
}

fn compare_and_swap<T, F>(xs: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mid_point = xs.len() / 2;
    for i in 0..mid_point {
        let ord = if up {
            Ordering::Greater
        } else {
            Ordering::Less
        };
        if (comparator(&xs[i], &xs[mid_point + i])) == ord {
            xs.swap(i, mid_point + i)
        }
    }
}

#[cfg(test)]
mod test {
    use super::sort;
    use super::sort_by;
    use crate::bitonic::SortOrder::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, Descending), Ok(()));
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
        assert_eq!(sort(&mut x, Ascending), Ok(()));
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
        assert_eq!(sort(&mut x, Descending), Ok(()));
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

    #[test]
    fn sort_student_by_age_ascending() {
        let miki = Student::new("Miki", "Hoshii", 15);
        let makoto = Student::new("Makoto", "Kikuchi", 17);
        let mami = Student::new("Mami", "Futami", 13);
        let ritsuko = Student::new("Ritsuko", "Akiduki", 19);
        let mut students = vec![&miki, &makoto, &mami, &ritsuko];
        let expected = vec![&mami, &miki, &makoto, &ritsuko];

        assert_eq!(sort_by(&mut students, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(students, expected);
    }

    #[test]
    fn sort_to_fail() {
        let mut xs = vec![10, 30, 1];
        assert!(sort(&mut xs, Ascending).is_err());
    }
}
