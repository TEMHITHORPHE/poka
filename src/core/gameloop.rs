use super::{
    engine::PokaEngine,
    shared::{GameStatus, Hand, PlayerAction, PokaVariant},
};

pub enum PlayerAction {
    Check,
    Bet(u32),
    Call,
    Raise(u32),
    Fold,
}

pub enum GameStatus {
    Paused,
    InPlay,
    NotStarted,
}

struct Player {
    game_id: u8,
    last_action: PlayAction,
    last_raise_amount: u32,
    purse_balance: u32,          // Amount of money player has.
    hand: Hand,
}

pub struct GameLoop {
    players: Vec<Player>,
    engine: PokaEngine,
    poker_variant: PokaVariant,
    pot_amount: u32,
    pot_limit: u32,        // pot limit of zero means no pot limit is set.
    min_raise_amount: u32, // must always have a non-zero value.
    max_raise_amount: u32, // a value of 0 means no set maximum, basically unlimited.
    last_player_action: PlayerAction,
    last_raised_amount: u32,
    status: GameStatus,
    dealer: u8,
    current_round: u8,
    player_count: u8,
}

impl GameLoop {
    fn new(poker_variant: PokaVariant, player_count: u8) -> Self {}

    fn start(mut self) {
        self.status = GameStatus::InPlay;
        
    }
}
