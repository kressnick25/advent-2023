advent_of_code::solution!(1);

fn first_digit(input: &str) -> u32 {
    for char in input.chars() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap()
        }
    }
    0
}

fn last_digit(input: &str) -> u32 {
    for char in input.chars().rev() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap()
        }
    }
    0
}

fn replace_numstrings(input: &str) -> String {
    let mut result;
    // keep first and last letter so we don't lose other words
    result = input.replace("one", "o1e");
    result = result.replace("two", "t2o");
    result = result.replace("three", "t3e");
    result = result.replace("four", "f4r");
    result = result.replace("five", "f5e");
    result = result.replace("six", "s6x");
    result = result.replace("seven", "s7n");
    result = result.replace("eight", "e8t");
    result = result.replace("nine", "n9e");

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let values = input.split('\n');

    let mut c: u32;
    let mut total: u32 = 0;

    for line in values {
        c = 0;

        c += first_digit(line) * 10;
        c += last_digit(line);

        total += c;
    }


    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let replaced = replace_numstrings(input);
    let values = replaced.split("\n");

    let mut c: u32;
    let mut total: u32 = 0;

    for line in values {
        c = 0;

        c += first_digit(line) * 10;
        c += last_digit(line);


        total += c;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
