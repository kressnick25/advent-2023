use advent_of_code::{extract_and_remove, find_numbers};
use regex::Regex;

advent_of_code::solution!(4);

struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    winning_count: Option<u32>
}

impl Card {
    fn new(winning: Vec<u32>, mine: Vec<u32>) -> Card {
        Card{winning_numbers: winning, my_numbers: mine, winning_count: None}
    }

    fn solve(&self) -> u32 {
        let winning_count: u32 = self.my_numbers.iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count()
            .try_into()
            .unwrap();

        winning_count
    }

     fn solve_memo(&mut self) -> u32 {
        if self.winning_count.is_none() {
            self.winning_count = Some(self.solve());
        }

        self.winning_count.unwrap()
    }
}

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
    let input = input.lines();

    let mut cards: Vec<Card> = vec![];

    let id_re = Regex::new(r"Card\s+(?<id>\d+): ").unwrap();
    for line in input {
        let (_, line) = extract_and_remove(line, &id_re, "id");

        let mut split = line.split("|");
        let input_winning = split.next().unwrap();
        let input_my_numbers = split.next().unwrap();
        assert!(input_my_numbers.len() > input_winning.len());

        let winning = find_numbers(input_winning);
        let mine = find_numbers(input_my_numbers);

        cards.push(Card::new(winning, mine));
    }
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter_mut().enumerate() {
        let mut copies = card_counts.get(i).unwrap().clone();
        while copies != 0 {
            let mut win_count = card.solve_memo();
            if win_count > 0 {
                while win_count != 0 {
                    let idx: usize = i + win_count as usize;

                    let next_card = card_counts.get_mut(idx);
                    if next_card.is_some() {
                        *next_card.unwrap() += 1;
                    }

                    win_count -= 1;
                }
            }
            copies -= 1;
        }
    }

    Some(card_counts.iter().sum())
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
        assert_eq!(result, Some(30));
    }
}
