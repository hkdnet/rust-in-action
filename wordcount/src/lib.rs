use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;

pub fn count(input: impl BufRead) -> HashMap <String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        for m in re.find_iter(&line){
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
        }
    }

    freqs
}
