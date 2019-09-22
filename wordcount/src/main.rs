use std::env;
use std::fs::File;
use std::io::BufReader;

use wordcount::count;

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME is required");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let frequencies = count(reader);
    println!("{:?}", frequencies);
}
