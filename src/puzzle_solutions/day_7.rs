use std::cmp::Ordering;

use crate::Problem;

pub struct DaySeven;

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();
        let mut games = Vec::new();

        for line in lines {
            let ls = line.trim().split(' ').collect::<Vec<_>>();
            let hand = Hand::new(ls[0], ls[1].parse().unwrap());
            games.push(hand)
        }

        // println!("Games before {:?}", games);
        // games.sort_by_key(|h| h.hand_type.clone());
        games.sort();
        // println!("Games after {:?}", games);

        let mut result = 0;

        for (i, game) in games.iter().enumerate() {
            println!("{}", game.bid);
            result += (i as u32 + 1) * game.bid
        }

        result.to_string()
    }
    fn part_two(&self, _input: &str) -> String {
        "This is just a template part two".to_string()
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        let mut h = Hand {
            cards: Vec::new(),
            bid,
            hand_type: HandType::HighCard,
        };
        for c in cards.chars() {
            if c.is_digit(10) {
                h.cards.push(c as u32 - 48)
            } else if c == 'A' {
                h.cards.push(14)
            } else if c == 'K' {
                h.cards.push(13)
            } else if c == 'Q' {
                h.cards.push(12)
            } else if c == 'J' {
                h.cards.push(11)
            } else if c == 'T' {
                h.cards.push(10)
            }
        }
        h.hand_type = h.get_hand_type();
        return h;
    }

    fn get_hand_type(&self) -> HandType {
        let mut max_count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..14 + 1 {
            let t = self.cards.iter().filter(|x| x == &&i).count();
            if t > max_count[i as usize] {
                max_count[i as usize] = t
            }
        }
        let m = max_count.iter().filter(|x| x > &&0).collect::<Vec<_>>();
        return match m.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if m[0] == &4 || m[1] == &4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if m[0] == &3 || m[1] == &3 || m[2] == &3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            for (i, card) in self.cards.iter().enumerate() {
                if card != &other.cards[i] {
                    return Some(card.cmp(&other.cards[i]));
                }
            }
        }
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hand_type() {
        assert_eq!(HandType::FiveOfAKind, Hand::new("11111", 0).get_hand_type());
        assert_eq!(HandType::FourOfAKind, Hand::new("1111A", 0).get_hand_type());
        assert_eq!(HandType::FullHouse, Hand::new("111AA", 0).get_hand_type());
        assert_eq!(
            HandType::ThreeOfAKind,
            Hand::new("11123", 0).get_hand_type()
        );
        assert_eq!(HandType::TwoPair, Hand::new("11AA2", 0).get_hand_type());
        assert_eq!(HandType::OnePair, Hand::new("11234", 0).get_hand_type());
        assert_eq!(HandType::HighCard, Hand::new("12345", 0).get_hand_type());
    }

    #[test]
    fn test_new_hand() {
        assert_eq!(vec![3, 2, 10, 3, 13], Hand::new("32T3K", 0).cards);
        assert_eq!(vec![10, 11, 12, 13, 14], Hand::new("TJQKA", 0).cards);
        assert_eq!(vec![2, 3, 4, 8, 9], Hand::new("23489", 0).cards);
    }

    #[test]
    fn test_hand_type_value() {
        let mut t = vec![
            HandType::FiveOfAKind as u32,
            HandType::FourOfAKind as u32,
            HandType::FullHouse as u32,
            HandType::ThreeOfAKind as u32,
            HandType::TwoPair as u32,
            HandType::OnePair as u32,
            HandType::HighCard as u32,
        ];
        t.sort();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], t);
    }
    #[test]
    fn test_part_one() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let day = DaySeven {};
        assert_eq!(day.part_one(input), "6440");
    }
    #[test]
    fn test_part_two() {
        let input = "This is just some input part two";

        let day = DaySeven {};
        assert_eq!(day.part_two(input), "This is just a template part two");
    }
}
