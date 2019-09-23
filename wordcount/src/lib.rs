//! wordcount provides simple features to count chars, words or lines.
//! Please see [`count`](fn.count.html) for detail.

#![deny(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// Options for [`count`](fn.count.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// Count by chars
    Char,
    /// Count by words
    Word,
    /// Count by lines
    Line,
}

/// The default value is [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// Count something from input and return frequencies.
///
/// You can pass option to change the count unit.
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): per a Unicode character
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): per a word matching `\w+`
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): per a line ending `\n` or `\r\n`
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();

        match option {
            CountOption::Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            CountOption::Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            CountOption::Line => {
                *freqs.entry(line).or_insert(0) += 1;
            }
        }
    }

    freqs
}

#[test]
fn word_count_works() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);
    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}
