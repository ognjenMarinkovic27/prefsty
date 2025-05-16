use super::{
    actions::{self, GameAction, GameActionKind},
    bidding::*,
    choosing::*,
    player::Player,
    playing::*,
    types::{Card, GameContract},
};

pub struct Room {
    pub game: GameState,
    players: Vec<Player>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            game: GameState::Starting(Game {
                state: StartingState {},
                first: 0,
                turn: 0,
                hands: Default::default(),
            }),
            players: Vec::new(),
        }
    }
}

pub struct Game<S> {
    pub state: S,
    pub first: usize,
    pub turn: usize,
    pub hands: [Vec<Card>; 3],
}

impl<S> Game<S> {
    pub fn validate_turn(&self, action: GameAction) -> bool {
        self.is_player_turn(action.player_ind)
    }

    fn is_player_turn(&self, player_ind: usize) -> bool {
        return self.turn == player_ind;
    }
}

pub enum GameState {
    Starting(Game<StartingState>),
    Bidding(Game<BiddingState>),
    NoBidPlayClaim(Game<NoBidClaimState>),
    NoBidPlayChoice(Game<NoBidChoiceState>),
    ChoosingCards(Game<ChoosingCardsState>),
    ChoosingContract(Game<ChoosingContractState>),
    RespondingToContract(Game<RespondingToContractState>),
    HelpOrContreToContract(Game<HelpOrContreToContractState>),
    ContreDeclared(Game<ContreDeclaredState>),
    Playing(Game<PlayingState>),
}

impl GameState {
    pub fn validate(&self, action: GameAction) -> bool {
        match self {
            GameState::Starting(game) => game.validate(action),
            GameState::Bidding(game) => game.validate(action),
            GameState::NoBidPlayClaim(game) => game.validate(action),
            GameState::NoBidPlayChoice(game) => game.validate(action),
            GameState::ChoosingCards(game) => game.validate(action),
            GameState::ChoosingContract(game) => game.validate(action),
            GameState::RespondingToContract(game) => game.validate(action),
            GameState::HelpOrContreToContract(game) => game.validate(action),
            GameState::ContreDeclared(game) => game.validate(action),
            GameState::Playing(game) => game.validate(action),
        }
    }
}

pub struct StartingState;

impl Game<StartingState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::Bid => true,
            _ => false,
        }
    }
}
