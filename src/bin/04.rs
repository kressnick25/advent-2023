use advent_of_code::{extract_and_remove, find_numbers};
use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines();
    let mut total: u32 = 0;

    let id_re = Regex::new(r"Card\s+(?<id>\d+): ").unwrap();
    for line in input {
        let (_, line) = extract_and_remove(line, &id_re, "id");

        let mut split = line.split("|");
        let input_winning = split.next().unwrap();
        let input_my_numbers = split.next().unwrap();
        assert!(input_my_numbers.len() > input_winning.len());

        let winning = find_numbers(input_winning);
        let my_numbers = find_numbers(input_my_numbers);

        let winning_numbers: u32 = my_numbers.iter()
            .filter(|&num| winning.contains(num))
            .count()
            .try_into()
            .unwrap();
        let base: u32 = 2;


        if winning_numbers == 1 {
            total += 1
        } else if winning_numbers > 1 {
            total += base.pow(winning_numbers - 1);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
