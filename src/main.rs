use rust_in_action::bitonic::SortOrder::{Ascending, Descending};

fn main() {
    match std::env::args().nth(1) {
        Some(s) => match s.as_ref() {
            "rpn" => rpn(),
            "bitonic" => bitonic(),
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
    use rust_in_action::bitonic;
    use rust_in_action::util;

    let mut arr = util::new_u32_vec(4);
    bitonic::sort(&mut arr, Ascending).expect("sortable");

    assert!(util::is_sorted(&arr, Ascending));
    println!("up = true : {:?}", arr);

    bitonic::sort(&mut arr, Descending).expect("sortable");

    assert!(util::is_sorted(&arr, Descending));
    println!("up = false: {:?}", arr);
}
