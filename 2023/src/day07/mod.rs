use std::collections::BTreeMap;

use advent::*;

advent_day!(Day07, parse, Vec<CamelHand<'a>>, part1, part2);

pub struct CamelHand<'a> {
    cards: &'a str,
    bid: u32,
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

/// ```rust
/// use advent_of_code_2023::day07::*;
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

        let score_bytes = [
            0,
            hand_strength,
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
/// use advent_of_code_2023::day07::*;
/// let input = parse(
/// r"32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483");
/// assert_eq!(5905, part2(&input));
/// ```
pub fn part2(input: &Vec<CamelHand>) -> u32 {
    const fn card_value(card: u8) -> u8 {
        match card {
            b'J' => 0x1,
            b'2' => 0x2,
            b'3' => 0x3,
            b'4' => 0x4,
            b'5' => 0x5,
            b'6' => 0x6,
            b'7' => 0x7,
            b'8' => 0x8,
            b'9' => 0x9,
            b'T' => 0xA,
            b'Q' => 0xC,
            b'K' => 0xD,
            b'A' => 0xE,
            _ => panic!("Invalid card"),
        }
    }

    fn kind_score(hand: &[u8]) -> u64 {
        let mut hand_strength = 0;
        let mut joker_count = 0;
        let mut max_strenth = 0;

        for i in 0..5 {
            if hand[i] == b'J' {
                joker_count += 1;
            } else {
                let mut card_strength = 0;
                for j in (i + 1)..5 {
                    if hand[i] == hand[j] {
                        card_strength += 1;
                    }
                }
                hand_strength += card_strength;
                max_strenth = max_strenth.max(card_strength);
            }
        }

        // Limit wildcard math to four card, the fifth joker is the card the other jokers match to.
        for _joker in 0..(joker_count.min(4)) {
            max_strenth += 1;
            hand_strength += max_strenth;
        }

        let score_bytes = [
            0,
            hand_strength,
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
