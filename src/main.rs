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

    let mut arr = [1u32, 2u32, 3u32, 4u32];
    bitonic::sort(&mut arr, Ascending);

    println!("up = true : {:?}", arr);

    bitonic::sort(&mut arr, Descending);

    println!("up = false: {:?}", arr);
}
