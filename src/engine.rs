// use core::todo;

// use core::iter::Iterator;

use std::clone;

use crate::shared::*;

#[derive(Debug)]
pub struct PokaEngine {
    pub variant: PokaVariant,
    u8_to_enum: [Rank; 14],
}

impl PokaEngine {
    pub fn new(variant: PokaVariant) -> PokaEngine {
        PokaEngine {
            variant,
            u8_to_enum: [
                Rank::None,
                Rank::Deuce,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ],
        }
    }

    pub fn evaluate(self, hands: &[Hand]) {
        match self.variant {
            PokaVariant::TexasHoldEm => PokaEngine::texas_hold_em(hands),
            PokaVariant::Stud => todo!(),
            PokaVariant::Omaha => todo!(),
            PokaVariant::SevenCardStud => todo!(),
        }
    }

    fn texas_hold_em(hands: &[Hand]) {
        // let
        let mut hand_rank_freq = [[0_u8, 0_u8], [0, 0], [0, 0], [0, 0], [0, 0]];
        for (i, hand) in hands.iter().enumerate() {
            // preliminary ops
            //
            // let mut cards_rank_weight = [
            //     (hand[0].rank as u8),
            //     (hand[1].rank as u8),
            //     (hand[2].rank as u8),
            //     (hand[3].rank as u8),
            //     (hand[4].rank as u8),
            // ];

            // ================= ==== ====== Weird Heap Free Implementation ===== ===== ===================
            // [(Rank, Frequency)]
            println!("\n==== Card {} ====", i);
            let mut j = 0_usize;
            for card in hand.iter() {
                let rank = card.rank as u8;
                println!("|{} as {:?}", rank, Rank::from(rank) );
                if rank == hand_rank_freq[0][0] {
                    hand_rank_freq[0] = [rank, hand_rank_freq[0][1] + 1];
                } else if rank == hand_rank_freq[1][0] {
                    hand_rank_freq[1] = [rank, hand_rank_freq[1][1] + 1];
                } else if rank == hand_rank_freq[2][0] {
                    hand_rank_freq[2] = [rank, hand_rank_freq[2][1] + 1];
                } else if rank == hand_rank_freq[3][0] {
                    hand_rank_freq[3] = [rank, hand_rank_freq[3][1] + 1];
                } else {
                    hand_rank_freq[i] = [rank, 1];
                    j += 1;
                }
            }

            println!("Card Freq: {:?}", hand_rank_freq);

            //
            // [EVALUATE]: HIGH

            // We reset our rank/frequency array.
            hand_rank_freq.fill([0_u8, 0_u8]);
            // ================= ==== ====== Weird Heap Free Implementation ===== ===== ===================

            // let sum_of_cards_rank_weight: u8 = cards_rank_weight[0]
            //     + cards_rank_weight[1]
            //     + cards_rank_weight[2]
            //     + cards_rank_weight[3]
            //     + cards_rank_weight[4];

            // basically sort each card by rank (ascending).
            // cards_rank_weight.sort();

            // Note:begin card evaluation from most probable (lowest) hand ranks first.
            //      perform exhaustive check for each hand rank individually,
            //      do not rely on code evaluation order.
            //
            // [EVALUATE]: HIGH
            // if ()
        }
    }
}
