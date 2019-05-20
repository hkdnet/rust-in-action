pub fn sort(xs: &mut [u32], up: bool) {
    if xs.len() > 1 {
        let mid_point = xs.len() / 2;
        sort(&mut xs[..mid_point], true);
        sort(&mut xs[mid_point..], false);
        sub_sort(xs, up)
    }
}

fn sub_sort(xs: &mut [u32], up: bool) {
    unimplemented!()
}

fn compare_and_swap(xs: &[u32], up: bool) {
    unimplemented!()
}
