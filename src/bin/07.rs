advent_of_code::solution!(7);

use std::{collections::{HashSet, HashMap}, cmp::Ordering};
use self::Card::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
}

impl Card {
    fn iterator() -> std::slice::Iter<'static, Card> {
        static CARDS: [Card; 14] = [Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two, One];
        CARDS.iter()
    }

    fn iterator_chars() -> std::slice::Iter<'static, char> {
        static CARDS: [char; 14] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1'];
        CARDS.iter()
    }


    fn cmp(&self, other: &Card) -> Ordering {
        self.as_int().cmp(&other.as_int())
    }

    fn as_int(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            '1' => Ok(One),
            _ => Err(())
        }
    }

}

#[derive(Copy, Clone)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    // fn interator() -> std::slice::Iter<'static, Card> {
        // static CARDS: [Card; 14] = [Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two, One];
        // CARDS.iter()
    // }

    fn cmp(&self, other: &HandType) -> Ordering {
        self.as_int().cmp(&other.as_int())
    }

    fn as_int(&self) -> usize {
        *self as usize
    }
}

struct Hand {
    cards: String,
    bid: usize
}

impl Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let mut ordering = self.get_type().cmp(&other.get_type());

        if ordering.is_eq() {
            // ordering = self.high_card().cmp(&other.high_card());

            // let mut ours = self.card_set_cards();
            // ours.sort_by(Card::cmp);
            // ours.dedup_by(|a, b| a.as_int() == b.as_int())

            // let mut theirs = other.card_set_cards();
            // ours.sort_by(Card::cmp);
            // ours.dedup_by(|a, b| a.as_int() == b.as_int())

            for (a, b) in std::iter::zip(self.card_set_cards(), other.card_set_cards()) {
                if a != b {
                    ordering = a.cmp(&b);
                    break;
                }
            }

        }

        ordering
    }

    fn high_card(&self) -> Card {
        let set = self.card_set();
        for i in Card::iterator_chars() {
            if set.contains(&i) { return Card::try_from(*i).unwrap() }
        }
        panic!("unknown high card")
    }

    fn get_type(&self) -> HandType {
        if self.is_five_of_a_kind() { return HandType::FiveOfAKind }
        if self.is_four_of_a_kind() { return HandType::FourOfAKind }
        if self.is_full_house() { return HandType::FullHouse }
        if self.is_three_of_a_kind() { return HandType::ThreeOfAKind }
        if self.is_two_pair() { return HandType::TwoPair }
        if self.is_one_pair() { return HandType::OnePair }
        if self.is_high_card() { return HandType::HighCard }

        else { panic!("Unknown HandType: {}", self.cards) }
    }

    fn card_set(&self) -> HashSet<char> {
        let chars = self.cards.chars().into_iter();
        HashSet::from_iter(chars)
    }

    fn card_set_cards(&self) -> Vec<Card> {
        self.cards.chars().map(|c| Card::try_from(c).unwrap()).collect()
    }

    fn card_map(&self) -> HashMap<char, i32> {
         let mut letter_counts: HashMap<char,i32> = HashMap::new();

        let char_vec: Vec<char> = self.cards.to_lowercase().chars().collect();
        for c in char_vec {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        letter_counts
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.card_set().len() == 1
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.card_map().values().any(|v| *v == 4)
    }

    fn is_full_house(&self) -> bool {
        self.card_map().values().any(|x| *x == 3) &&
        self.card_map().values().any(|x| *x == 2)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.card_map().values().any(|x| *x == 3) &&
        !self.card_map().values().any(|x| *x == 2)
    }

    fn is_two_pair(&self) -> bool {
        let card_map = self.card_map();
        let c: Vec<&i32> = card_map.values().filter(|x| **x == 2).collect();
        c.len() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.card_map().values().any(|x| *x == 2) &&
        !self.card_map().values().any(|x| *x == 3)
    }

    fn is_high_card(&self) -> bool {
        self.card_set().len() == 5
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            let cards = s.next().unwrap();
            let bid = s.next().unwrap();
            let bid = bid.parse::<usize>().unwrap();

            Hand{cards: cards.to_string(), bid: bid}
        })
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        let multiplyer = i + 1;
        total += hand.bid * multiplyer;
    }

    Some(total as u32)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
