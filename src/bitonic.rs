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
