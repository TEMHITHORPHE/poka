mod engine;
mod shared;

use engine::PokaEngine;
use shared::*;

fn main() {
    println!("Hello, world!");
    let texas = PokaEngine::new(PokaVariant::TexasHoldEm);
    let stud = PokaEngine::new(PokaVariant::Stud);
    let cardOne = Card::new(Rank::King, Suit::Hearts);
    let cardTwo = Card::new(Rank::Jack, Suit::Diamonds);
    let handOne = [
        Card::new(Rank::Jack, Suit::Diamonds),
        Card::new(Rank::Six, Suit::Spades),
        Card::new(Rank::Jack, Suit::Diamonds),
        Card::new(Rank::Deuce, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Hearts),
    ];
    // println!("{:?} \n{:?}", texas, stud);
    // println!("{:?} ", Card { suit: Suit::} + stud);
    // println!("{:?}",);
    texas.evaluate(&[handOne]);
}
