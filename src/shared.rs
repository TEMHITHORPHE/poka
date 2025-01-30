// use core::ops::Add;
// use std::process::Output;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Rank {
    None = 0,
    Deuce = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        match value {
            2 => Rank::Deuce,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => Rank::None,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

// #[derive(Debug)]
#[derive(PartialEq, PartialOrd, Debug)]
pub enum PokaVariant {
    TexasHoldEm,
    Stud,
    Omaha,
    SevenCardStud,
}

pub type Hand = [Card; 5];

enum HandRank {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

enum HandRankingResult {
    StraightFlush(),
}
