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
    let hand_straight: Hand = [
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Diamonds),
        Card::new(Rank::Eight, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Spades),
    ];
    let hand_flush: Hand = [
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Hearts),
    ];
    let hand_full_house: Hand = [
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::King, Suit::Clubs),
    ];
    let hand_four_of_a_kind: Hand = [
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Ace, Suit::Hearts),
    ];
    let hand_straight_flush: Hand = [
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Queen, Suit::Hearts),
    ];
    let hand_rand1: Hand = [
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Deuce, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Queen, Suit::Clubs),
    ];
    let hand_rand2: Hand = [
        Card::new(Rank::Three, Suit::Spades),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::King, Suit::Spades),
        Card::new(Rank::Seven, Suit::Hearts),
        Card::new(Rank::Deuce, Suit::Clubs),
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
            hand_straight,
            hand_flush,
            hand_full_house,
            hand_four_of_a_kind,
            hand_straight_flush,
            hand_rand1,
            hand_rand2
        ]
        .each_ref(),
    );
}
