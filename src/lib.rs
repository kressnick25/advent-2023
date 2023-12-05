mod day;
pub mod template;

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
