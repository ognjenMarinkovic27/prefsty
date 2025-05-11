use super::{player::Player, states::{bidding::BiddingState, playing::PlayingState}};

pub struct Game {
    state: GameState,
    players: Vec<Player>,
}

pub enum GameState {
    Starting,
    Bidding(BiddingState),
    ChoosingCards,
    ChoosingContract,
    Playing(PlayingState)
}

pub enum GameContract {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
    Betl,
    Sans
}