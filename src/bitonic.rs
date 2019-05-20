pub fn sort(xs: &mut [u32], up: bool) {
    if xs.len() > 1 {
        let mid_point = xs.len() / 2;
        sort(&mut xs[..mid_point], true);
        sort(&mut xs[mid_point..], false);
        sub_sort(xs, up)
    }
}

fn sub_sort(xs: &mut [u32], up: bool) {
    if xs.len() > 1 {
        compare_and_swap(xs, up);
        let mid_point = xs.len() / 2;
        sub_sort(&mut xs[..mid_point], up);
        sub_sort(&mut xs[mid_point..], up)
    }
}

fn compare_and_swap(xs: &mut [u32], up: bool) {
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

    #[test]
    fn sort_u32_ascending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, true);
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
}
