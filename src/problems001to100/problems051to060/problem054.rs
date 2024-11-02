// https://projecteuler.net/problem=54

use std::cmp::{max, min, Ordering, Reverse};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const POKER_PATH: &str = "./src/problems001to100/problems051to060/problem054_poker.txt";

type Card = u8;

const SUIT_MASK: Card = 0x30;
const HEARTS: Card = 0x00;
const DIAMONDS: Card = 0x10;
const CLUBS: Card = 0x20;
const SPADES: Card = 0x30;

const VALUE_MASK: Card = 0xF;
const T: Card = 10;
const J: Card = 11;
const Q: Card = 12;
const K: Card = 13;
const A: Card = 14;

#[derive(Debug, Clone, Copy)]
enum HandEvaluation {
    StraightFlush(Card),
    FourOfAKind([Card; 2]),
    FullHouse([Card; 2]),
    Flush([Card; 5]),
    Straight(Card),
    ThreeOfAKind([Card; 3]),
    TwoPair([Card; 3]),
    OnePair([Card; 4]),
    HighCard([Card; 5]),
}
#[derive(Debug, Clone, Copy)]
enum Pairings {
    Pair(Card),
    ThreeOfAKind(Card),
    FourOfAKind(Card),
    Junk(Card),
}

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Poker Hands",
        number: 54,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    read_hands(POKER_PATH)
        .iter()
        .map(|(hand1, hand2)| first_hand_wins((*hand1, *hand2)))
        .filter(|a| *a)
        .count() as i64
}

fn first_hand_wins(hands: ([Card; 5], [Card; 5])) -> bool {
    let hand1 = evaluate_hand(hands.0);
    let hand2 = evaluate_hand(hands.1);

    match hand1 {
        HandEvaluation::StraightFlush(high_card1) => {
            if let HandEvaluation::StraightFlush(high_card2) = hand2 {
                high_card1 > high_card2
            } else {
                true
            }
        }
        HandEvaluation::FourOfAKind(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::FullHouse(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::Flush(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(_) => false,
            HandEvaluation::Flush(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::Straight(card1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(_) => false,
            HandEvaluation::Flush(_) => false,
            HandEvaluation::Straight(card2) => card1 > card2,
            _ => true,
        },
        HandEvaluation::ThreeOfAKind(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(_) => false,
            HandEvaluation::Flush(_) => false,
            HandEvaluation::Straight(_) => false,
            HandEvaluation::ThreeOfAKind(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::TwoPair(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(_) => false,
            HandEvaluation::Flush(_) => false,
            HandEvaluation::Straight(_) => false,
            HandEvaluation::ThreeOfAKind(_) => false,
            HandEvaluation::TwoPair(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::OnePair(cards1) => match hand2 {
            HandEvaluation::StraightFlush(_) => false,
            HandEvaluation::FourOfAKind(_) => false,
            HandEvaluation::FullHouse(_) => false,
            HandEvaluation::Flush(_) => false,
            HandEvaluation::Straight(_) => false,
            HandEvaluation::ThreeOfAKind(_) => false,
            HandEvaluation::TwoPair(_) => false,
            HandEvaluation::OnePair(cards2) => first_cards_win(&cards1, &cards2),
            _ => true,
        },
        HandEvaluation::HighCard(cards1) => {
            if let HandEvaluation::HighCard(cards2) = hand2 {
                first_cards_win(&cards1, &cards2)
            } else {
                false
            }
        }
    }
}

fn first_cards_win(cards1: &[Card], cards2: &[Card]) -> bool {
    if cards1.len() != cards2.len() {
        panic!();
    }

    for (card1, card2) in cards1.iter().zip(cards2.iter()) {
        if card1 & VALUE_MASK > card2 & VALUE_MASK {
            return true;
        }
        if card1 & VALUE_MASK < card2 & VALUE_MASK {
            return false;
        }
    }
    // tie means first did not win, but should be unreachable
    false
}

fn evaluate_hand(hand: [Card; 5]) -> HandEvaluation {
    let straight_result = is_straight(hand);
    let flush_result = is_flush(hand);
    if let Some(high_card) = straight_result {
        if flush_result.is_some() {
            return HandEvaluation::StraightFlush(high_card);
        }
    }

    let pairings = find_pairings(hand);
    if let Some(cards) = is_four_of_a_kind(&pairings) {
        return HandEvaluation::FourOfAKind(cards);
    }
    if let Some(cards) = is_full_house(&pairings) {
        return HandEvaluation::FullHouse(cards);
    }
    if let Some(cards) = flush_result {
        return HandEvaluation::Flush(cards);
    }
    if let Some(high_card) = straight_result {
        return HandEvaluation::Straight(high_card);
    }
    if let Some(cards) = is_three_of_a_kind(&pairings) {
        return HandEvaluation::ThreeOfAKind(cards);
    }
    if let Some(cards) = is_two_pair(&pairings) {
        return HandEvaluation::TwoPair(cards);
    }
    if let Some(cards) = is_one_pair(&pairings) {
        return HandEvaluation::OnePair(cards);
    }
    let sorted_hand = sort_cards(hand);
    HandEvaluation::HighCard([
        sorted_hand[0],
        sorted_hand[1],
        sorted_hand[2],
        sorted_hand[3],
        sorted_hand[4],
    ])
}
fn sort_cards(hand: [Card; 5]) -> [Card; 5] {
    let mut sorted_hand: Vec<Card> = hand.iter().map(|card| card & VALUE_MASK).collect();
    sorted_hand.sort_by_key(|card| Reverse(*card));
    [
        sorted_hand[0],
        sorted_hand[1],
        sorted_hand[2],
        sorted_hand[3],
        sorted_hand[4],
    ]
}

fn find_pairings(hand: [Card; 5]) -> Vec<Pairings> {
    let mut pairings = Vec::new();
    let mut value_map = [0; (A as usize) + 1];
    for card in hand {
        let card_value = (card & VALUE_MASK) as usize;
        value_map[card_value] += 1;
    }

    for value in 2..=A {
        match value_map[value as usize] {
            4 => pairings.push(Pairings::FourOfAKind(value)),
            3 => pairings.push(Pairings::ThreeOfAKind(value)),
            2 => pairings.push(Pairings::Pair(value)),
            1 => pairings.push(Pairings::Junk(value)),
            _ => {}
        }
    }

    pairings.sort_by(|pairing1, pairing2| match pairing1 {
        Pairings::FourOfAKind(value1) => match pairing2 {
            Pairings::FourOfAKind(value2) => value1.cmp(value2),
            _ => Ordering::Greater,
        },
        Pairings::ThreeOfAKind(value1) => match pairing2 {
            Pairings::FourOfAKind(_) => Ordering::Less,
            Pairings::ThreeOfAKind(value2) => value1.cmp(value2),
            _ => Ordering::Greater,
        },
        Pairings::Pair(value1) => match pairing2 {
            Pairings::Junk(_) => Ordering::Greater,
            Pairings::Pair(value2) => value1.cmp(value2),
            _ => Ordering::Less,
        },
        Pairings::Junk(value1) => match pairing2 {
            Pairings::Junk(value2) => value1.cmp(value2),
            _ => Ordering::Less,
        },
    });
    pairings.reverse();

    pairings
}

fn is_four_of_a_kind(pairings: &[Pairings]) -> Option<[Card; 2]> {
    if pairings.len() == 2 {
        if let Pairings::FourOfAKind(quadruple) = pairings[0] {
            if let Pairings::Junk(junk) = pairings[1] {
                return Some([quadruple, junk]);
            }
        }
    }
    None
}

fn is_full_house(pairings: &[Pairings]) -> Option<[Card; 2]> {
    if pairings.len() == 2 {
        if let Pairings::ThreeOfAKind(triple) = pairings[0] {
            if let Pairings::Pair(pair) = pairings[1] {
                return Some([triple, pair]);
            }
        }
    }
    None
}

fn is_flush(hand: [Card; 5]) -> Option<[Card; 5]> {
    if hand
        .iter()
        .skip(1)
        .all(|card| card & SUIT_MASK == hand[0] & SUIT_MASK)
    {
        let sorted_hand = sort_cards(hand);
        Some([
            sorted_hand[0] & VALUE_MASK,
            sorted_hand[1] & VALUE_MASK,
            sorted_hand[2] & VALUE_MASK,
            sorted_hand[3] & VALUE_MASK,
            sorted_hand[4] & VALUE_MASK,
        ])
    } else {
        None
    }
}

fn is_straight(hand: [Card; 5]) -> Option<Card> {
    let mut value_map = [false; A as usize + 1];

    let mut min_card_value = (A as usize) + 1;
    let mut max_card_value = 0;

    for card in hand {
        let card_value = (card & VALUE_MASK) as usize;
        // if previously encountered card, there are duplicates => not a straight
        if value_map[card_value] {
            return None;
        }
        value_map[card_value] = true;

        max_card_value = max(max_card_value, card_value);
        min_card_value = min(min_card_value, card_value);
    }

    if max_card_value - min_card_value == 4 {
        Some(max_card_value as u8)
    } else {
        None
    }
}

fn is_three_of_a_kind(pairings: &[Pairings]) -> Option<[Card; 3]> {
    if pairings.len() == 3 {
        if let Pairings::ThreeOfAKind(triple) = pairings[0] {
            if let Pairings::Junk(junk1) = pairings[1] {
                if let Pairings::Junk(junk2) = pairings[2] {
                    return Some([triple, junk1, junk2]);
                }
            }
        }
    }
    None
}

fn is_two_pair(pairings: &[Pairings]) -> Option<[Card; 3]> {
    if pairings.len() == 3 {
        if let Pairings::Pair(pair1) = pairings[0] {
            if let Pairings::Pair(pair2) = pairings[1] {
                if let Pairings::Junk(junk) = pairings[2] {
                    return Some([pair1, pair2, junk]);
                }
            }
        }
    }
    None
}

fn is_one_pair(pairings: &[Pairings]) -> Option<[Card; 4]> {
    if pairings.len() == 4 {
        if let Pairings::Pair(pair1) = pairings[0] {
            if let Pairings::Junk(junk1) = pairings[1] {
                if let Pairings::Junk(junk2) = pairings[2] {
                    if let Pairings::Junk(junk3) = pairings[3] {
                        return Some([pair1, junk1, junk2, junk3]);
                    }
                }
            }
        }
    }
    None
}

fn parse_number(raw_card: &str) -> Card {
    match raw_card.as_bytes()[0] {
        b'T' => T,
        b'J' => J,
        b'Q' => Q,
        b'K' => K,
        b'A' => A,
        other => other - b'0',
    }
}
fn parse_suit(raw_card: &str) -> Card {
    match raw_card.as_bytes()[1] {
        b'C' => CLUBS,
        b'S' => SPADES,
        b'H' => HEARTS,
        b'D' => DIAMONDS,
        _ => 0,
    }
}
fn parse_card(raw_card: &str) -> Card {
    parse_number(raw_card) | parse_suit(raw_card)
}
fn parse_match(line: String) -> ([Card; 5], [Card; 5]) {
    let cards: Vec<&str> = line.split(' ').collect();
    (
        [
            parse_card(cards[0]),
            parse_card(cards[1]),
            parse_card(cards[2]),
            parse_card(cards[3]),
            parse_card(cards[4]),
        ],
        [
            parse_card(cards[5]),
            parse_card(cards[6]),
            parse_card(cards[7]),
            parse_card(cards[8]),
            parse_card(cards[9]),
        ],
    )
}
fn read_hands(path: &str) -> Vec<([Card; 5], [Card; 5])> {
    let path = Path::new(path);
    let file = match File::open(path) {
        Err(_) => return vec![],
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    reader
        .lines()
        .map(|line| parse_match(line.unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::problems001to100::problems051to060::problem054::{first_hand_wins, parse_match};

    #[test]
    fn toy_example() {
        assert!(!first_hand_wins(parse_match(String::from(
            "5H 5C 6S 7S KD 2C 3S 8S 8D TD"
        ))));
        assert!(first_hand_wins(parse_match(String::from(
            "5D 8C 9S JS AC 2C 5C 7D 8S QH"
        ))));
        assert!(!first_hand_wins(parse_match(String::from(
            "2D 9C AS AH AC 3D 6D 7D TD QD"
        ))));
        assert!(first_hand_wins(parse_match(String::from(
            "4D 6S 9H QH QC 3D 6D 7H QD QS"
        ))));
        assert!(first_hand_wins(parse_match(String::from(
            "2H 2D 4C 4D 4S 3C 3D 3S 9S 9D"
        ))));
    }

    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 376)
    }
}
