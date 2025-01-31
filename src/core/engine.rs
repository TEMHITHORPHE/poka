use std::array;

use crate::core::shared::*;

// pub type Hand = &[Card; 5];
type HandEvaluationResult<'a> = (HandId, &'a Hand, HandRank);
type HandId = u8;

#[derive(Debug)]
pub struct PokaEngine {
    pub variant: PokaVariant,
}

impl PokaEngine {
    pub fn new(variant: PokaVariant) -> PokaEngine {
        PokaEngine { variant }
    }

    pub fn evaluate(self, hands: &[&Hand]) {
        let hands_ranking = match self.variant {
            PokaVariant::TexasHoldEm => PokaEngine::texas_hold_em(hands.clone()),
            PokaVariant::Stud => todo!(),
            PokaVariant::Omaha => todo!(),
            PokaVariant::SevenCardStud => todo!(),
        };
    }

    fn texas_hold_em<'a>(players_hands: &'a [&'a Hand]) -> Vec<HandEvaluationResult<'a>> {
        let mut hand_ranks = Vec::<HandEvaluationResult>::with_capacity(players_hands.len());
        println!("Player Hands: {}", players_hands.len());
        let mut hand_rank_freq = [[0_u8, 0_u8], [0, 0], [0, 0], [0, 0], [0, 0]];

        // need a sorted player hand to make certain logic evaluation easier.
        let mut sorted_players_hand: [u8; 5] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
        println!("Sorted Players Hands: {}", sorted_players_hand.len());

        for (hand_idx, hand) in players_hands.iter().enumerate() {
            println!("\n==== Card {} ====", hand_idx + 1);
            //
            // ================= ==== ====== Weird Heap Free Implementation ===== ===== ===================
            // preliminary ops
            //
            sorted_players_hand[0] = hand[0].rank as u8;
            sorted_players_hand[1] = hand[1].rank as u8;
            sorted_players_hand[2] = hand[2].rank as u8;
            sorted_players_hand[3] = hand[3].rank as u8;
            sorted_players_hand[4] = hand[4].rank as u8;

            sorted_players_hand.sort_by(|rank_a, rank_b| rank_a.cmp(rank_b));
            // structure below is just me avoiding to use an hashmap or similar construct (heap avoidance).
            // [ (Rank, Frequency) ]
            let mut j = 0_usize;
            for card in hand.iter() {
                let rank = card.rank as u8;
                // sorted_players_hand[]
                // code below counts the frequency of occurrence of a card in a particular hand
                if rank == hand_rank_freq[0][0] {
                    hand_rank_freq[0] = [rank, hand_rank_freq[0][1] + 1];
                } else if rank == hand_rank_freq[1][0] {
                    hand_rank_freq[1] = [rank, hand_rank_freq[1][1] + 1];
                } else if rank == hand_rank_freq[2][0] {
                    hand_rank_freq[2] = [rank, hand_rank_freq[2][1] + 1];
                } else if rank == hand_rank_freq[3][0] {
                    hand_rank_freq[3] = [rank, hand_rank_freq[3][1] + 1];
                } else {
                    hand_rank_freq[j] = [rank, 1];
                    j += 1;
                }
                println!("|{} as {:?}", rank, Rank::from(rank));
            }

            // this allows for easy comparisons and logic evaluation of ranks by frequency.
            hand_rank_freq.sort_by(|arr_one, arr_two| arr_two[1].cmp(&arr_one[1]));

            //
            // [EVALUATE]: HIGH
            if hand_rank_freq[0][1] == 1
                && hand_rank_freq[1][1] == 1
                && hand_rank_freq[2][1] == 1
                && hand_rank_freq[3][1] == 1
                && hand_rank_freq[4][1] == 1
            {
                hand_ranks.push((hand_idx as u8, hand, HandRank::High));
                println!("|\n|=> {:?} ", HandRank::High);
            }
            //
            //
            // [EVALUATE]: One Pair
            if hand_rank_freq[0][1] == 2_u8 && hand_rank_freq[1][1] != 2_u8 {
                hand_ranks.push((hand_idx as u8, hand, HandRank::Pair));
                println!("|\n|=> {:?} ", HandRank::Pair);
            }
            //
            //
            // [EVALUATE]: Two Pair
            if hand_rank_freq[0][1] == 2_u8 && hand_rank_freq[1][1] == 2_u8 {
                hand_ranks.push((hand_idx as u8, hand, HandRank::TwoPair));
                println!("|\n|=> {:?} ", HandRank::TwoPair);
            }
            //
            //
            // [EVALUATE]: Three Of A Kind
            if hand_rank_freq[0][1] == 3_u8 && hand_rank_freq[1][1] == 1_u8 {
                hand_ranks.push((hand_idx as u8, hand, HandRank::ThreeOfAKind));
                println!("|\n|=> {:?} ", HandRank::ThreeOfAKind);
            }
            //
            //
            // [EVALUATE]: Straight
            if (sorted_players_hand[0] + 1 == sorted_players_hand[1]
                && sorted_players_hand[1] + 1 == sorted_players_hand[2]
                && sorted_players_hand[2] + 1 == sorted_players_hand[3]
                && sorted_players_hand[3] + 1 == sorted_players_hand[4])
                && (hand[0].suit != hand[1].suit
                    || hand[1].suit != hand[2].suit
                    || hand[2].suit != hand[3].suit
                    || hand[3].suit != hand[4].suit)
            {
                hand_ranks.push((hand_idx as u8, hand, HandRank::Straight));
                println!("|\n|=> {:?} ", HandRank::Straight);
            }

            println!("Card Freq: {:?}", hand_rank_freq);
            println!("Card Sorted: {:?}", sorted_players_hand);

            // We reset our rank/frequency array.
            hand_rank_freq.fill([0_u8, 0_u8]);
        }

        return hand_ranks;

        // ================= ==== ====== Weird Heap Free Implementation ===== ===== ===================
        //
        //
        // let mut cards_rank_weight = [
        //     (hand[0].rank as u8),
        //     (hand[1].rank as u8),
        //     (hand[2].rank as u8),
        //     (hand[3].rank as u8),
        //     (hand[4].rank as u8),
        // ];

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
