mod day;
pub mod template;

use regex::Regex;


pub use day::*;


pub fn digits(input: &str) -> Vec<u32> {
    input.chars()
        .into_iter()
        .filter_map(|c| match c.is_digit(10) {
            true => Some(c.to_digit(10).unwrap()),
            false => None
        })
        .collect()
}

pub fn extract_and_remove(input: &str, re: &Regex, group: &str) -> (String, String) {
    let caps = re.captures(input).unwrap();
    let extracted = caps[group].to_string();

    let removed = re.replace(input, "").to_string();

    (extracted, removed)
}


pub fn find_numbers(s: &str) -> Vec<u32> {
    // FIXME make this regex 'static'
    let num_re: Regex = Regex::new(r"\d+").unwrap();

    // iterate over all matches
    num_re.find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}
