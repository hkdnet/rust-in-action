use rust_in_action::bitonic;
use rust_in_action::bitonic::SortOrder::{Ascending, Descending};
use rust_in_action::util;
use std::time::Instant;

fn main() {
    match std::env::args().nth(1) {
        Some(s) => match s.as_ref() {
            "rpn" => rpn(),
            "bitonic" => bitonic(),
            "bitonic_benchmark" => bitonic_benchmark(23),
            e => println!("Unknown arg: {}", e),
        },
        None => eprintln!("arg is required"),
    }
}

fn rpn() {
    use rust_in_action::rpn;

    rpn::rpn();
}

fn bitonic() {
    let mut arr = util::new_u32_vec(4);
    bitonic::sort(&mut arr, Ascending).expect("sortable");

    assert!(util::is_sorted(&arr, Ascending));
    println!("up = true : {:?}", arr);

    bitonic::sort(&mut arr, Descending).expect("sortable");

    assert!(util::is_sorted(&arr, Descending));
    println!("up = false: {:?}", arr);
}

fn bitonic_benchmark(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;
    println!(
        "sorting {} integers ({:.1}MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );
    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let parallel_sorter = bitonic::Sorter::new(4096);
    let parallel_ns = timed_sort(len, parallel_sorter);
    println!(
        "parallel: sorted {} integers in {} seconds",
        len,
        parallel_ns / 1e9
    );

    let seq_sorter = bitonic::Sorter::new(len + 1);
    let seq_ns = timed_sort(len, seq_sorter);
    println!("seq: sorted {} integers in {} seconds", len, seq_ns / 1e9);

    println!("speed up: {:.2}x", seq_ns / parallel_ns);
}
fn timed_sort(len: usize, sorter: bitonic::Sorter) -> f64 {
    let mut xs = util::new_u32_vec(len);

    let start = Instant::now();
    sorter.sort(&mut xs, Ascending).expect("sorted");
    let dur = start.elapsed();

    assert!(util::is_sorted(&mut xs, Ascending));

    dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64
}
