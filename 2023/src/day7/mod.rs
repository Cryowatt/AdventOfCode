use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    mem::transmute,
    slice,
    sync::Arc,
};

use advent::*;

advent_day!(Day7, parse, Vec<CamelHand>, part1, part2);

pub struct CamelHand<'a> {
    cards: &'a str,
    bid: u32,
}

impl std::hash::Hash for CamelHand<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cards.hash(state);
        self.bid.hash(state);
    }
}

pub fn parse(input: &str) -> Vec<CamelHand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            CamelHand {
                cards,
                bid: bid.parse::<u32>().unwrap(),
            }
        })
        .collect()
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

/// ```rust
/// use advent_of_code_2023::day7::*;
/// let input = parse(
/// r"32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483");
/// assert_eq!(6440, part1(&input));
/// ```
pub fn part1(input: &Vec<CamelHand>) -> u32 {
    const fn card_value(card: u8) -> u8 {
        match card {
            b'2' => 0x2,
            b'3' => 0x3,
            b'4' => 0x4,
            b'5' => 0x5,
            b'6' => 0x6,
            b'7' => 0x7,
            b'8' => 0x8,
            b'9' => 0x9,
            b'T' => 0xA,
            b'J' => 0xB,
            b'Q' => 0xC,
            b'K' => 0xD,
            b'A' => 0xE,
            _ => panic!("Invalid card"),
        }
    }

    fn kind_score(hand: &[u8]) -> u64 {
        let mut hand_strength = 0;

        for i in 0..5 {
            for j in (i + 1)..5 {
                if hand[i] == hand[j] {
                    hand_strength += 1;
                }
            }
        }

        let kind = match hand_strength {
            10 => HandType::FiveKind,
            6 => HandType::FourKind,
            4 => HandType::FullHouse,
            3 => HandType::ThreeKind,
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            0 => HandType::HighCard,
            _ => {
                println!("FK {}", hand_strength);
                unreachable!()
            }
        };

        let score_bytes = [
            0,
            kind as u8,
            0,
            card_value(hand[0]),
            card_value(hand[1]),
            card_value(hand[2]),
            card_value(hand[3]),
            card_value(hand[4]),
        ];

        u64::from_be_bytes(score_bytes)
    }

    let mut scores: BTreeMap<u64, u32> = BTreeMap::new();

    input.iter().for_each(|hand| {
        let score = kind_score(hand.cards.as_bytes());
        scores.insert(score, hand.bid);
    });

    let mut rank = 0;
    scores
        .iter()
        .map(|score_entry| {
            rank += 1;
            rank * score_entry.1
        })
        .sum()
}

/// ```rust
/// use advent_of_code_2023::day7::*;
/// let input = parse(
/// r"32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483");
/// // assert_eq!(71503, part2(&input));
/// ```
pub fn part2(input: &Vec<CamelHand>) -> u32 {
    const fn card_value(card: u8) -> u8 {
        match card {
            b'2' => 0x2,
            b'3' => 0x3,
            b'4' => 0x4,
            b'5' => 0x5,
            b'6' => 0x6,
            b'7' => 0x7,
            b'8' => 0x8,
            b'9' => 0x9,
            b'T' => 0xA,
            b'J' => 0xB,
            b'Q' => 0xC,
            b'K' => 0xD,
            b'A' => 0xE,
            _ => panic!("Invalid card"),
        }
    }

    let mut scores: BTreeMap<u64, u32> = BTreeMap::new();
    input.iter().for_each(|hand| {
        let mut score_bytes = [0u8; 8];
        for card in hand.cards.bytes().map(card_value) {
            for i in 0..8 {
                match (score_bytes[i] & 0xf).cmp(&card) {
                    Ordering::Less => {
                        score_bytes.copy_within(i..7, i + 1);
                        score_bytes[i] = card;
                        break;
                    }
                    Ordering::Equal => {
                        score_bytes[i] = score_bytes[i] + 0x10;
                        break;
                    }
                    Ordering::Greater => continue,
                }
            }
        }
        score_bytes.sort();
        let score: u64 = u64::from_le_bytes(score_bytes);
        scores.insert(score, hand.bid);
        // })
    });

    let mut rank = 0;
    scores
        .iter()
        .map(|score_entry| {
            rank += 1;
            rank * score_entry.1
        })
        .sum()
}
