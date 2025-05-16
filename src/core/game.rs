use super::{
    bidding::*,
    choosing::*,
    player::Player,
    playing::*,
    types::{Card, GameContract},
};

pub struct Room {
    game: Game,
    players: Vec<Player>,
}

pub struct Game {
    pub state: GameState,
    pub first: usize,
    pub turn: usize,
    pub hands: [Vec<Card>; 3],
}

pub enum GameState {
    Starting,
    Bidding(BiddingState),
    NoBidPlayClaim(NoBidClaimState),
    NoBidPlayChoice(NoBidChoiceState),
    ChoosingCards(GameContract),
    ChoosingContract(GameContract),
    RespondingToContract(RespondingToContractState),
    HelpOrContreToContract(RespondingToContractState),
    ContreDeclared(ContreDeclaredState),
    Playing(PlayingState),
}
