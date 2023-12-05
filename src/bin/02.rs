advent_of_code::solution!(2);

use regex::Regex;

#[derive(Debug)]
struct Game {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>
}

impl Game {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        let mut possible = true;
        if self.red.is_some() {
            possible = possible && self.red.unwrap() <= red;
        }
        if self.green.is_some() {
            possible = possible && self.green.unwrap() <= green;
        }
        if self.blue.is_some() {
            possible = possible && self.blue.unwrap() <= blue;
        }
        possible
    }

    fn from_str(input: &str) -> Game {
        let lines = input.split(",");

        let mut red: Option<u32> = None;
        let mut blue: Option<u32> = None;
        let mut green: Option<u32> = None;
        let re = Regex::new(r"(?P<score>\d+) (?P<color>[a-z]+)").unwrap();

        for line in lines {
            let caps = re.captures(line.trim()).unwrap();

            let score: &u32 = &caps["score"].parse().unwrap();
            match &caps["color"] {
                "red" => red = Some(score.to_owned()),
                "green" => green = Some(score.to_owned()),
                "blue" => blue = Some(score.to_owned()),
                _ => panic!("colour not found")
            }
        }

        Game{red, green, blue}
    }

}

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    let re = Regex::new(r"Game (?<id>\d+): ").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let id: u32 = caps["id"].parse().unwrap();

        let line = re.replace(line, "");

        let games_input: Vec<&str> = line.split(";").collect();
        let mut games = games_input
            .into_iter()
            .map(|i| {
                let i = i.trim();
                Game::from_str(i)
            });

        if !games.any(|g| !g.is_possible(RED_MAX, GREEN_MAX, BLUE_MAX)) {
            total += id;
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
