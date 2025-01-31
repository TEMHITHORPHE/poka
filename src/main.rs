mod core;

use crate::core::engine::PokaEngine;
use crate::core::shared::*;

fn main() {
    println!("Hello, world!");
    let texas = PokaEngine::new(PokaVariant::TexasHoldEm);
    let stud = PokaEngine::new(PokaVariant::Stud);
    let card_one = Card::new(Rank::King, Suit::Hearts);
    let card_two = Card::new(Rank::Jack, Suit::Diamonds);
    let hand_high: Hand = [
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Eight, Suit::Spades),
    ];
    let hand_one_pair: Hand = [
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Deuce, Suit::Clubs),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Seven, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Diamonds),
    ];
    let hand_two_pair: Hand = [
        Card::new(Rank::Six, Suit::Spades),
        Card::new(Rank::Deuce, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Deuce, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Clubs),
    ];
    let hand_three_of_a_kind: Hand = [
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Nine, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Diamonds),
        Card::new(Rank::Nine, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Spades),
    ];
    // println!("{:?} \n{:?}", texas, stud);
    // println!("{:?} ", Card { suit: Suit::} + stud);
    // println!("{:?}",);
    texas.evaluate(
        &[
            hand_high,
            hand_one_pair,
            hand_two_pair,
            hand_three_of_a_kind,
        ]
        .each_ref(),
    );
}
